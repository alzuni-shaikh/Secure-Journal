use sqlx::SqlitePool;
use anyhow::Result;

pub type DbPool = SqlitePool;

pub async fn connect() -> Result<DbPool> {
    let pool = SqlitePool::connect("sqlite://data/journal.db").await?;
    Ok(pool)
}
