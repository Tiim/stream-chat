use crate::source::Event;

use anyhow::Result;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use tokio::sync::broadcast::Receiver;
use uuid::Uuid;

pub struct SqliteDestination {
    rx: Receiver<Event>,
    db: Pool<Sqlite>,
}

impl SqliteDestination {
    pub fn new(rx: Receiver<Event>, db: &Pool<Sqlite>) -> Self {
        SqliteDestination { rx, db: db.clone() }
    }
    pub async fn run(mut self) -> Result<String> {
        loop {
            let event = self.rx.recv().await?;
            let mut conn = self.db.acquire().await?;

            let event_str = serde_json::to_string(&event)?;
            let id = Uuid::new_v4().to_string();
            let ts = Utc::now().to_rfc3339();

            sqlx::query!(
                "INSERT INTO events (id, ts, data) VALUES (?, ?, ?);",
                id,
                ts,
                event_str,
            )
            .execute(&mut conn)
            .await?;
        }
    }
}
