mod auth;
mod flashcards;
mod state;
mod stats;
mod words;
mod ws;

use axum::Router;
use axum::http::{HeaderValue, Method};
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

    let origins: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173".to_string())
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("invalid origin in CORS_ORIGINS"))
        .collect();
    info!("CORS origins: {:?}", origins);

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

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server is listening on {}", bind_addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
