use crate::source::{ChatEvent, ChatSource, Event};

use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
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
            let conn = self.db.acquire().await?;

            sqlx::query!(
                "INSERT INTO events (id, data) VALUES (?, ?);",
                Uuid::new_v4(),
                event,
            )
            .fetch(conn);
        }
    }
}
