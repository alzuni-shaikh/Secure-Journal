mod auth;
mod db;
mod models;
mod common;

use anyhow::Result;

use db::connect;
use common::utils::main_menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Secure Journal App ;)");

    let db = connect().await?;
    main_menu(&db).await;

    Ok(())
}


//tomorrow's tasks:
// organized menu (optional)
// 
// export journal to .md or .pdf using pdf crate
// sync journal enteries with cloud storage or self-hosted server
// integrate axum which integrates with surrealdb
// add a UI framework like dioxus/yew 
// 