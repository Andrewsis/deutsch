mod auth;
mod flashcards;
mod state;
mod stats;
mod words;
mod ws;

use axum::Router;
use axum::http::Method;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::routing::{get, post};
use sqlx::mysql::MySqlPoolOptions;
use state::AppState;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    info!("Starting database connection...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    info!("Database connected successfully.");

    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "http://127.0.0.1:5173".parse().unwrap(),
        "http://192.168.1.71:5173".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE, ACCEPT]);

    let state = AppState {
        pool,
        waiting_player: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/api/word/{word}", get(words::get_word))
        .route("/api/examples", get(words::get_examples))
        .route("/api/register", post(auth::register))
        .route("/api/login", post(auth::login))
        .route("/api/flashcards/due", get(flashcards::get_due_flashcards))
        .route("/api/flashcards/review", post(flashcards::review_flashcard))
        .route("/api/ws/match", get(ws::ws_handler))
        .route("/api/stats/words", get(stats::get_word_stats))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Server is listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
