use chrono::Utc;
use tokio::sync::mpsc::UnboundedSender;

use crate::source::{self, ChatEvent, Event, ChatSource};
use twitchchat::{connector::tokio::Connector, messages, runner::AsyncRunner, Status, UserConfig};

use anyhow::Result;

pub struct TwitchSource {
    runner: AsyncRunner,
    tx: UnboundedSender<Event>,
}

impl TwitchSource {
    pub async fn new(tx: UnboundedSender<Event>, channel: String) -> Result<Self> {
        let user_config = UserConfig::builder().anonymous().build()?;
        let connector = Connector::twitch()?;

        let mut runner = AsyncRunner::connect(connector, &user_config).await?;
        runner.join(channel.as_str()).await?;
        tx.send(Event::Info { msg: format!("joined channel {channel}"), src: Some(ChatSource::Twitch) })?;
        Ok(TwitchSource { runner, tx })
    }
    pub async fn run(mut self) -> anyhow::Result<String> {
        loop {
            let status = self.runner.next_message().await?;
            match status {
                Status::Message(msg) => handle_message(&self.tx, msg),
                Status::Quit => {}
                Status::Eof => {}
            }
        }
    }
}

fn handle_message(tx: &UnboundedSender<Event>, msg: messages::Commands<'_>) {
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
