use crate::source::{Command, Event};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", content = "settings")]
pub enum ActivatedCommands {
    TTS {
        max_length: usize,
    },
}
pub struct CommandMiddleware {
    rx: Receiver<Event>,
    tx: Sender<Event>,
    cmds: Vec<ActivatedCommands>,
}

impl CommandMiddleware {
    pub fn new(tx: Sender<Event>, rx: Receiver<Event>, cmds: Vec<ActivatedCommands>) -> Self {
        CommandMiddleware { rx, tx, cmds }
    }
    pub async fn run(mut self) -> Result<String> {
        loop {
            let event = self.rx.recv().await?;
            match event {
                Event::Chat { chat } => {
                    if let Some(cmd) = parse_commands(&self.cmds, chat.message.as_str()) {
                        self.tx.send(Event::Command { cmd })?;
                    }
                }
                _ => continue,
            }
        }
    }
}

fn parse_commands(cmds: &Vec<ActivatedCommands>, str: &str) -> Option<Command> {
    let str_trim = str.trim();
    for cmd in cmds {
        let prefix = cmd.prefix();
        if str_trim.starts_with(prefix) {
            return cmd.get_command(str_trim.chars().skip(prefix.len()).collect());
        }
    }
    return None;
}

impl ActivatedCommands {
    fn prefix(&self) -> &'static str {
        match self {
            Self::TTS { max_length: _ } => "!tts ",
        }
    }
    fn get_command(&self, args: String) -> Option<Command> {
        match self {
            Self::TTS { max_length } => {
                if args.len() <= *max_length {
                    Some(Command::TTS(args))
                } else {
                    None
                }
            }
        }
    }
}
