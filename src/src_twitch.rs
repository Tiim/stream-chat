use chrono::Utc;
use tokio::sync::mpsc::UnboundedSender;

use crate::source::{self, ChatEvent, Event};
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

        Ok(TwitchSource { runner, tx })
    }
    pub async fn run(mut self) -> anyhow::Result<()> {
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
