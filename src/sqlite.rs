use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use sqlx::{migrate::Migrator, Pool, Sqlite, SqlitePool};
use xdg::BaseDirectories;

const DB_NAME: &str = "stream-chat/data.db";
pub async fn get_database(db: Option<&str>) -> Result<Pool<Sqlite>> {
    let db_file = match db {
        None => {
            let dbdir = BaseDirectories::new()?;
            dbdir.place_data_file(DB_NAME)?
        }
        Some(dbf) => PathBuf::from_str(dbf)?,
    };
    let pool =
        SqlitePool::connect(format!("sqlite://{}", db_file.to_str().unwrap()).as_str()).await?;
    let migrator: Migrator = sqlx::migrate!();
    migrator.run(&pool).await?;
    return Ok(pool);
}
