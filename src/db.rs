use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub async fn connect() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    let _ = db
        .signin(Root {
            username: "root",
            password: "secret",
        })
        .await;
    db.use_ns("journal").use_db("database").await.unwrap();
    
    Ok(db)
}