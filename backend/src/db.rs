use sqlx::{SqlitePool, Sqlite};
use anyhow::Result;

pub type DbPool = SqlitePool;

pub async fn connect() -> Result<DbPool> {
    let pool = SqlitePool::connect("sqlite://journal.db").await?;
    Ok(pool)
}
