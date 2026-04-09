use axum::extract::ws::WebSocket;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

pub type WaitingPlayer = Arc<Mutex<Option<(oneshot::Sender<(WebSocket, String)>, String)>>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub waiting_player: WaitingPlayer,
}
