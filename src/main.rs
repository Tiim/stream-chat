use std::time::Duration;
use colored::{Colorize, ColoredString};

use tokio::{task, time};
use youtube_chat::{live_chat::LiveChatClientBuilder, item::{ChatItem, MessageItem::Text, MessageItem::Emoji}};

#[tokio::main]
async fn main() {
    let mut client = LiveChatClientBuilder::new()
        .url("https://www.youtube.com/watch?v=iLqlRq1Kmg4".to_string())
        .unwrap()
        .on_chat(|chat_item|print_chat_message(chat_item))
        .on_error(|error| eprintln!("{:?}", error))
        .build();
    client.start().await.unwrap();
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(3000));
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

    let obracket = "<".green();
    let cbracket = ">".green();

    if let Some(author_name) = author.name {
        print!("{}{}{} ", obracket, author_name.cyan(), cbracket);
    }
    let full_text = txt
        .iter()
        .map(|t| {
            match t {
                Text(s) => s,
                Emoji(_) => "", 
            }
        }).fold(String::new(), |a,b| a+b+"\n");
    print!("{}", full_text.blue());
}
