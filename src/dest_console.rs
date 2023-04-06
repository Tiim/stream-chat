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
    pub async fn run(mut self) -> Result<()> {
        while let Some(event) = self.rx.recv().await {
            match event {
                Event::Chat { chat } => print_chat(chat),
                Event::Error { err } => println!("{}", err),
            }
        }
        Err(anyhow::format_err!("Can't receive any more chat events"))
    }
}

fn print_chat(ce: ChatEvent) {
    let src = match ce.src {
        ChatSource::YoutubeLive => "Yt",
        ChatSource::Twitch => "Tw",
    };
    println!("{} <{}> {}", src, ce.author, ce.message);
}
