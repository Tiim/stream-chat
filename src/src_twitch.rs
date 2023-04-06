use chrono::Utc;
use tokio::sync::mpsc::UnboundedSender;

use crate::source::{self, ChatEvent, Event};
use twitchchat::{connector::tokio::Connector, messages, runner::AsyncRunner, Status, UserConfig};

use anyhow::Result;

pub struct TwitchSource {
    user_config: UserConfig,
    connector: Connector,
    channel: String,
    tx: UnboundedSender<Event>,
}

impl TwitchSource {
    pub fn new(tx: UnboundedSender<Event>, channel: String) -> Result<Self> {
        let user_config = UserConfig::builder().anonymous().build().unwrap();
        let connector = Connector::twitch()?;
        Ok(TwitchSource {
            user_config,
            connector,
            channel,
            tx,
        })
    }
    pub async fn run(self) -> anyhow::Result<()> {
        let mut runner = AsyncRunner::connect(self.connector, &self.user_config).await?;
        runner.join(self.channel.as_str()).await?;
        loop {
            let status = runner.next_message().await?;
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
