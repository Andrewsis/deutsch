use axum::extract::ws::WebSocket;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

/// What the second player hands to the waiting one: socket, username, translation language.
pub type Opponent = (WebSocket, String, &'static str);

pub type WaitingPlayer = Arc<Mutex<Option<(oneshot::Sender<Opponent>, String)>>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub waiting_player: WaitingPlayer,
}
