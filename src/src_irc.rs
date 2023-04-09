use crate::source::{ChatEvent, ChatSource, Event};
use anyhow::Result;
use chrono::Utc;
use futures::stream::StreamExt;
use irc::{
    client::prelude::{Client, Config},
    proto::Command,
};
use tokio::sync::mpsc::UnboundedSender;

pub struct IrcSource {
    tx: UnboundedSender<Event>,
    client: Client,
}

impl IrcSource {
    pub async fn new(
        tx: UnboundedSender<Event>,
        nick: String,
        server: String,
        channel: String,
    ) -> Result<Self> {
        let client = Client::from_config(Config {
            nickname: Some(nick),
            server: Some(server),
            ..Config::default()
        })
        .await?;
        client.identify()?;
        client.send_join(channel.to_owned())?;
        return Ok(IrcSource { tx, client });
    }

    pub async fn run(mut self) -> anyhow::Result<String> {
        let mut stream = self.client.stream()?;
        while let Some(message) = stream.next().await.transpose()? {
            let author = message.source_nickname().unwrap_or("anon").to_owned();
            match message.command {
                Command::PRIVMSG(_channel, msg) => {
                    self.tx.send(Event::Chat {
                        chat: ChatEvent {
                            src: crate::source::ChatSource::IRC,
                            ts: Utc::now(),
                            author,
                            message: msg,
                        },
                    })?;
                }

                Command::JOIN(chanlist, _chankeys, real_name) => {
                    self.tx.send(Event::Info {
                        msg: format!("joined channel {}", real_name.unwrap_or(chanlist)),
                        src: Some(ChatSource::IRC),
                    })?;
                }
                _ => {}
            }
        }
        return Ok("IrcSource".to_owned());
    }
}
