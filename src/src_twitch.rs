use chrono::Utc;
use tokio::sync::broadcast::Sender;

use crate::source::{self, ChatEvent, ChatSource, Event};
use twitchchat::{connector::tokio::Connector, messages, runner::AsyncRunner, Status, UserConfig};

use anyhow::{Context, Result};

pub struct TwitchSource {
    runner: AsyncRunner,
    tx: Sender<Event>,
}

impl TwitchSource {
    pub async fn new(tx: Sender<Event>, channel: String) -> Result<Self> {
        let user_config = UserConfig::builder()
            .anonymous()
            .build()
            .with_context(|| "Failed to build Twitch user config")?;
        let connector = Connector::twitch().with_context(|| "Failed to create twitch connector")?;

        let mut runner = AsyncRunner::connect(connector, &user_config)
            .await
            .with_context(|| "Failed to create twitch connection runner")?;
        runner
            .join(channel.as_str())
            .await
            .with_context(|| format!("Failed to connect to twich channel {}", channel))?;

        tx.send(Event::Info {
            msg: format!("joined channel {channel}"),
            src: Some(ChatSource::Twitch),
        })?;

        Ok(TwitchSource { runner, tx })
    }
    pub async fn run(mut self) -> anyhow::Result<String> {
        loop {
            let status = self
                .runner
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
