use crate::source::{ChatEvent, ChatSource, Event};
use anyhow::{Result, Context};
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
    server: String,
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
            server: Some(server.to_owned()),
            ..Config::default()
        })
        .await.with_context(||format!("Failed to connect to IRC server: {}", server))?;
        client.identify().with_context(|| format!("Failed identify to IRC server: {}", server))?;
        client.send_join(channel.to_owned()).with_context(||format!("Failed join IRC channels on server: {}", server))?;
        return Ok(IrcSource { tx, client, server });
    }

    pub async fn run(mut self) -> anyhow::Result<String> {
        let mut stream = self.client.stream().with_context(|| format!("Failed to stream IRC messages from server {}", self.server))?;
        while let Some(message) = stream.next().await.transpose().with_context(|| format!("Failed getting next IRC message from server {}", self.server))? {
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
