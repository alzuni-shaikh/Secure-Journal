mod auth;
mod common;
mod db;
mod helpers;
mod models;

use common::utils::main_menu;
use db::DbPool;
use std::sync::Arc;
use axum::{Server, Router, routing::post};
use tokio::net::TcpListener;

use crate::auth::http::{login_handler::api_login, signup_handler::api_signup};

pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::connect().await?;
    let state = Arc::new(AppState { db });

    tokio::spawn(start_server(state.clone()));

    main_menu(&state.db).await;

    Ok(())
}

pub async fn start_server(state: Arc<AppState>) {
    let app = Router::new()
        .route("/api/login", post(api_login))
        .route("/api/signup", post(api_signup))
        .with_state(state);

    let _listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind port");

    println!("HTTP API running on http://localhost:8000");

    Server::bind(&"0.0.0.0:9000".parse().unwrap()) //8000
    .serve(app.into_make_service())
    .await
    .unwrap();
    // axum::serve(listener, app)
    //     .await
    //     .unwrap();
}
