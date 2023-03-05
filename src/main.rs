use std::{time::Duration, env};
use colored::Colorize;

use tokio::{task, time};
use youtube_chat::{live_chat::LiveChatClientBuilder, item::{ChatItem, MessageItem::Text, MessageItem::Emoji, EmojiItem}};
use demoji::demoji;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let stream_url: String;
    if args.len() == 1 {
        println!("WARNING: using default stream url!");
        stream_url = "https://www.youtube.com/watch?v=jfKfPfyJRdk".to_string()
    }
    else if args.len() > 2 {
        println!("Usage {} <Stream Url>", args[0]);
        return;
    } else {
        stream_url = args[1].to_string();
    }

    println!("Starting youtube chat!");
    let mut client = LiveChatClientBuilder::new()
        .url(stream_url)
        .unwrap()
        .on_chat(|chat_item|print_chat_message(chat_item))
        .on_error(|error| eprintln!("{:?}", error))
        .build();
    client.start().await.unwrap();
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1000));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    forever.await.unwrap();
}


fn print_chat_message(msg: ChatItem) {
    let author = msg.author;
    let txt = msg.message;

    // let bar = "|".truecolor(100,100,100);

    if let Some(author_name) = author.name {
        // print!("{:<10.10}{}", demoji(&author_name).green().bold(), bar);
        print!("{:.16} ", demoji(&author_name).green().bold());
    }
    let full_text = txt
        .iter()
        .map(|t| {
            match t {
                Text(s) => s.clone(),
                Emoji(emoji) => emoji_to_text(emoji), 
            }
        }).fold(String::new(), |a,b| {
            if b.trim().len() == 0 {
                a
            } else {
                a+&b
            }
        });
    print!("{}\n", full_text.blue());
}

fn emoji_to_text(emoji: &EmojiItem) -> String {
    let mut text = String::new();
    if let Some(emj_text) = emoji.emoji_text.clone() {
        text.push_str(&emj_text);
    }
    else if let Some(img) = emoji.image_item.clone() {
        if let Some(alt) = img.alt {
            text.push_str(&alt);
        }
    }
    text
}






