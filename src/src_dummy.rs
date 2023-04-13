use std::time::Duration;

use crate::source::{ChatEvent, ChatSource, Event};
use anyhow::Result;
use chrono::Utc;
use tokio::{sync::broadcast::Sender, time::sleep};

pub struct DummySource {
    tx: Sender<Event>,
}

impl DummySource {
    pub async fn new(tx: Sender<Event>) -> Result<Self> {
        return Ok(DummySource { tx });
    }

    pub async fn run(self) -> anyhow::Result<String> {
        loop {
            self.tx.send(Event::Chat {
                chat: ChatEvent {
                    src: ChatSource::Dummy,
                    ts: Utc::now(),
                    author: "Tiim".to_string(),
                    message: "Dummy message!!!".to_string(),
                },
            })?;
            sleep(Duration::from_secs(4)).await;
        }
    }
}
