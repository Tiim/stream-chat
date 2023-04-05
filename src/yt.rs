use std::time::Duration;

use anyhow::{Error, Result};
use chrono::Utc;
use demoji::demoji;

use tokio::{sync::mpsc::UnboundedSender, task, time};

use youtube_chat::{
    item::{ChatItem, EmojiItem, MessageItem},
    live_chat::LiveChatClientBuilder,
};

use crate::source::{self, ChatEvent, Event};

pub struct YoutubeSource {
    stream_url: String,
    tx: UnboundedSender<Event>,
}

impl YoutubeSource {
    pub fn new(tx: UnboundedSender<Event>, stream_url: &str) -> Result<Self> {
        return Ok(YoutubeSource {
            stream_url: stream_url.to_string(),
            tx,
        });
    }
    fn send_message(tx: UnboundedSender<Event>, chat_item: ChatItem) {
        if let Err(e) = tx.send(format_message(chat_item)) {
            println!("Error when sending new chat message to consumer: {}", e);
        }
    }
    fn send_error(tx: UnboundedSender<Event>, err: Error) {
        if let Err(e) = tx.send(Event::Error {
            err: err.to_string(),
        }) {
            println!("Error when sending recv error to consumer: {}", e);
        }
    }
}

impl source::Source for YoutubeSource {
    fn run(self) -> task::JoinHandle<()> {
        task::spawn(async move {
            let mut client = LiveChatClientBuilder::new()
                .url(self.stream_url)
                .unwrap()
                .on_chat(|ci| Self::send_message(self.tx.clone(), ci))
                .on_error(|err| Self::send_error(self.tx.clone(), err))
                .build();
            client.start().await.unwrap();
            let mut interval = time::interval(Duration::from_millis(1000));
            loop {
                interval.tick().await;
                client.execute().await;
            }
        })
    }
}

fn format_message(msg: ChatItem) -> Event {
    let author;
    let txt = msg.message;

    // let bar = "|".truecolor(100,100,100);

    if let Some(author_name) = msg.author.name {
        // print!("{:<10.10}{}", demoji(&author_name).green().bold(), bar);
        author = format!("{:.16} ", demoji(&author_name));
    } else {
        author = "@".to_string();
    }
    let full_text = txt
        .iter()
        .map(|t| match t {
            MessageItem::Text(s) => s.clone(),
            MessageItem::Emoji(emoji) => emoji_to_text(emoji),
        })
        .fold(
            String::new(),
            |a, b| {
                if b.trim().len() == 0 {
                    a
                } else {
                    a + &b
                }
            },
        );
    Event::Chat {
        chat: ChatEvent {
            message: format!("{}\n", full_text),
            author,
            ts: Utc::now(),
        },
    }
}

fn emoji_to_text(emoji: &EmojiItem) -> String {
    let mut text = String::new();
    if let Some(emj_text) = emoji.emoji_text.clone() {
        text.push_str(&emj_text);
    } else if let Some(img) = emoji.image_item.clone() {
        if let Some(alt) = img.alt {
            text.push_str(&alt);
        }
    }
    text
}
