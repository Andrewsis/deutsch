pub mod game;

use axum::extract::{Query, State, WebSocketUpgrade};
use tokio::sync::oneshot;
use tracing::{error, info, warn};

use crate::auth::Claims;
use crate::lang::LangQuery;
use crate::state::{AppState, Opponent};

#[tracing::instrument(skip(ws, state))]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    Query(lang): Query<LangQuery>,
    State(state): State<AppState>,
) -> axum::response::Response {
    let username = claims.sub;
    let lang = lang.code();
    info!("Player {} ({}) is connecting to WebSocket", username, lang);
    ws.on_upgrade(move |socket| handle_socket(socket, username, lang, state))
}

async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    username: String,
    lang: &'static str,
    state: AppState,
) {
    let (tx, rx) = oneshot::channel::<Opponent>();

    let mut lobby = state.waiting_player.lock().await;
    if lobby.is_none() {
        info!("{} entered lobby", username);
        *lobby = Some((tx, username.clone()));
        drop(lobby);

        match rx.await {
            Ok((opponent_socket, opponent_name, opponent_lang)) => {
                tokio::spawn(game::run_game(
                    socket,
                    username,
                    opponent_socket,
                    opponent_name,
                    [lang, opponent_lang],
                    state.pool,
                ));
            }
            Err(_) => {
                warn!("Lobby channel closed without opponent");
            }
        }
    } else {
        let (opponent_tx, opponent_name) = lobby.take().unwrap();
        drop(lobby);

        info!("Match found: {} vs {}", opponent_name, username);
        if opponent_tx.send((socket, username, lang)).is_err() {
            error!("Failed to wake up waiting player");
        }
    }
}
