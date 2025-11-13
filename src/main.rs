mod auth;
mod db;
mod menu;
mod models;

use db::connect;
use menu::main_menu;

#[tokio::main]
async fn main() {
    println!("Welcome to Secure Journal App ;)");

    let db = connect().await.expect("Failed to connect to database");
    
    main_menu(&db).await;
}


//tomorrow's tasks:
// organized menu (optional)
// 
// database and storage enchancements like timestamps, tags, soft delete
// export journal to .md or .pdf using pdf crate
// sync journal enteries with cloud storage or self-hosted server
// integrate axum which integrates with surrealdb
// add a UI framework like dioxus/yew 
// 