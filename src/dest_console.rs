use crate::source::{ChatEvent, ChatSource, Event};

use anyhow::Result;
use tokio::sync::broadcast::Receiver;

pub struct ConsoleDestination {
    rx: Receiver<Event>,
}

impl ConsoleDestination {
    pub fn new(rx: Receiver<Event>) -> Self {
        ConsoleDestination { rx }
    }
    pub async fn run(mut self) -> Result<String> {
        loop {
            let event = self.rx.recv().await?;
            match event {
                Event::Chat { chat } => print_chat(chat),
                Event::Error { err } => println!("{}", err),
                Event::Info { msg, src } => eprintln!(
                    "<{}> {}",
                    src.map_or("".to_owned(), |e| format!("{:?}", e)),
                    msg
                ),
            }
        }
    }
}

fn print_chat(ce: ChatEvent) {
    let src = match ce.src {
        ChatSource::YoutubeLive => "Yt ",
        ChatSource::Twitch => "TTv",
        ChatSource::IRC => "IRC",
        ChatSource::Dummy => "DMY",
    };
    println!("{} <{}> {}", src, ce.author, ce.message);
}
