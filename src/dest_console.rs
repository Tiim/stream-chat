use crate::source::{ChatEvent, ChatSource, Event};

use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct ConsoleDestination {
    rx: UnboundedReceiver<Event>,
}

impl ConsoleDestination {
    pub fn new(rx: UnboundedReceiver<Event>) -> Self {
        ConsoleDestination { rx }
    }
    pub async fn run(mut self) -> Result<String> {
        while let Some(event) = self.rx.recv().await {
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
        return Ok("ConsoleDestination".to_owned());
    }
}

fn print_chat(ce: ChatEvent) {
    let src = match ce.src {
        ChatSource::YoutubeLive => "Yt ",
        ChatSource::Twitch => "TTv",
        ChatSource::IRC => "IRC",
    };
    println!("{} <{}> {}", src, ce.author, ce.message);
}
