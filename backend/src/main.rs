mod auth;
mod common;
mod db;
mod helpers;
mod models;

use auth::api::{login_api, signup_api};
use axum::extract::{Json, State};
use common::utils::main_menu;
use db::DbPool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::{Server, Router, routing::post};
use tokio::net::TcpListener;

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


#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    ok: bool,
    message: String,
}

async fn api_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match login_api(&state.db, &payload.username, &payload.password).await {
        Ok(Some(user)) => Json(AuthResponse {
            ok: true,
            message: format!("Logged in as {}", user.username),
        }),
        Ok(None) => Json(AuthResponse {
            ok: false,
            message: "Invalid credentials".into(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
}

async fn api_signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match signup_api(&state.db, &payload.username, &payload.password).await {
        Ok(()) => Json(AuthResponse {
            ok: true,
            message: "Signup successful".into(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
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
