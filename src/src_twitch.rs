use chrono::Utc;
use tokio::sync::broadcast::Sender;

use crate::source::{self, ChatEvent, ChatSource, Event};
use twitchchat::{connector::tokio::Connector, messages, runner::AsyncRunner, Status, UserConfig};

use anyhow::{Context, Result};

pub struct TwitchSource {
    channel: String,
    tx: Sender<Event>,
}

impl TwitchSource {
    pub async fn new(tx: Sender<Event>, channel: String) -> Result<Self> {
        Ok(TwitchSource { channel, tx })
    }
    pub async fn run(self) -> anyhow::Result<String> {
        let user_config = UserConfig::builder()
            .anonymous()
            .build()
            .with_context(|| "Failed to build Twitch user config")?;
        let connector = Connector::twitch().with_context(|| "Failed to create twitch connector")?;

        let mut retries = 0;
        let mut runner: AsyncRunner;
        loop {
            let res = AsyncRunner::connect(connector.clone(), &user_config)
                .await
                .with_context(|| "Failed to create twitch connection runner");

            match res {
                Ok(run) => {
                    runner = run;
                    break;
                }
                Err(e) => {
                    retries += 1;
                    if retries > 5 {
                        return Err(e);
                    }
                }
            }
        }
        runner
            .join(self.channel.as_str())
            .await
            .with_context(|| format!("Failed to connect to twich channel {}", self.channel))?;

        self.tx.send(Event::Info {
            msg: format!("joined channel {}", self.channel),
            src: Some(ChatSource::Twitch),
        })?;

        loop {
            let status = runner
                .next_message()
                .await
                .with_context(|| "Failed to retrieve next message from twitch")?;
            match status {
                Status::Message(msg) => handle_message(&self.tx, msg),
                Status::Quit => {}
                Status::Eof => {}
            }
        }
    }
}

fn handle_message(tx: &Sender<Event>, msg: messages::Commands<'_>) {
    use messages::Commands::*;
    match msg {
        Privmsg(msg) => {
            tx.send(Event::Chat {
                chat: ChatEvent {
                    src: source::ChatSource::Twitch,
                    author: msg.name().to_string(),
                    ts: Utc::now(),
                    message: msg.data().to_string(),
                },
            })
            .unwrap();
        }
        _ => {}
    }
}
