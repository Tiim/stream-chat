

use crate::source::{Command, Event};

use anyhow::Result;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct CommandMiddleware {
    rx: Receiver<Event>,
    tx: Sender<Event>,
}

impl CommandMiddleware {
    pub fn new(tx: Sender<Event>, rx: Receiver<Event>) -> Self {
        CommandMiddleware { rx, tx }
    }
    pub async fn run(mut self) -> Result<String> {
        loop {
            let event = self.rx.recv().await?;
            match event {
                Event::Chat { chat } => {
                    if let Some(cmd) = parse_commands(chat.message.as_str()) {
                        self.tx.send(Event::Command { cmd })?;
                    }
                }
                _ => continue,
            }
        }
    }
}

fn parse_commands(str: &str) -> Option<Command> {
    let str_trim = str.trim();
    if !str_trim.starts_with("!") {
        return None;
    }
    if str_trim.starts_with("!tts ") {
        return Some(Command::TTS(str_trim.chars().skip(5).collect()));
    }
    return None;
}
