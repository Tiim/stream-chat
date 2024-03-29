use std::time::Duration;

use anyhow::{Context, Error, Result};
use chrono::Utc;
use demoji::demoji;

use tokio::{sync::broadcast::Sender, time};

use youtube_chat::{
    item::{ChatItem, EmojiItem, MessageItem},
    live_chat::LiveChatClientBuilder,
};

use crate::source::{ChatEvent, ChatSource, Event};

pub struct YoutubeSource {
    stream_url: String,
    tx: Sender<Event>,
}

impl YoutubeSource {
    pub async fn new(tx: Sender<Event>, channel_username: String) -> Result<Self> {
        return Ok(YoutubeSource {
            stream_url: format!("https://www.youtube.com/{}/live", channel_username),
            tx,
        });
    }
    fn send_message(tx: Sender<Event>, chat_item: ChatItem) {
        if let Err(e) = tx.send(format_message(chat_item)) {
            println!("Error when sending new chat message to consumer: {}", e);
        }
    }
    fn send_error(tx: Sender<Event>, err: Error) {
        if let Err(e) = tx.send(Event::Error {
            err: err.to_string(),
        }) {
            println!("Error when sending recv error to consumer: {}", e);
        }
    }
    pub async fn run(self) -> Result<String> {
        let stream_url = self.stream_url.clone();
        let mut client = LiveChatClientBuilder::new()
            .url(self.stream_url)
            .with_context(|| format!("Invalid url {}", stream_url))?
            .on_chat(|ci| Self::send_message(self.tx.clone(), ci))
            .on_error(|err| Self::send_error(self.tx.clone(), err))
            .build();
        client.start().await.with_context(|| {
            format!("Failed to start youtube chat client for url {}", stream_url)
        })?;
        let mut interval = time::interval(Duration::from_millis(1000));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    }
}

fn format_message(msg: ChatItem) -> Event {
    let author;
    let txt = msg.message;

    if let Some(author_name) = msg.author.name {
        author = format!("{:.16}", demoji(&author_name));
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
            src: ChatSource::YoutubeLive,
            ts: Utc::now(),
            author,
            message: format!("{}", full_text),
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
