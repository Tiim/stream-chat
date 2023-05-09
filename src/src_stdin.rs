use crate::source::{ChatEvent, ChatSource, Event};
use anyhow::Result;
use chrono::Utc;
use futures::StreamExt;
use tokio::io;
use tokio::sync::broadcast::Sender;
use tokio_util::codec::{FramedRead, LinesCodec};





pub struct StdinSource {
    tx: Sender<Event>,
}

impl StdinSource {
    pub async fn new(tx: Sender<Event>) -> Result<Self> {
        return Ok(StdinSource { tx });
    }

    pub async fn run(self) -> anyhow::Result<String> {
        let mut stdin = FramedRead::new(io::stdin(), LinesCodec::new());

        loop {
            let line = stdin.next().await.transpose()?;
            if let Some(line) = line {
                self.tx.send(Event::Chat {
                    chat: ChatEvent {
                        src: ChatSource::Stdin,
                        ts: Utc::now(),
                        author: "Tiim".to_string(),
                        message: line.trim().to_owned(),
                    },
                })?;
            }
        }
    }
}
