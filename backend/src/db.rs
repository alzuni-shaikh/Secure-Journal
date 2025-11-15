use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};
use anyhow::Result;

pub async fn connect() -> Result<Surreal<Db>> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("app").use_db("journal").await?;

    println!("Connected to in-memory SurrealDB");
    Ok(db)
}
