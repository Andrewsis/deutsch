use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;
use sqlx::prelude::FromRow;
use tracing::error;

use crate::state::AppState;

#[derive(Serialize, FromRow)]
pub struct WordStatRow {
    pub word: String,
    pub is_bigram: bool,
    pub count: i32,
}

pub async fn get_word_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let latest_run = sqlx::query_scalar!("SELECT MAX(scraped_at) FROM word_stats")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            error!("Failed to get latest scraped_at: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let Some(latest) = latest_run else {
        return Ok(Json(serde_json::json!({ "words": [], "bigrams": [] })));
    };

    let words = sqlx::query_as!(
        WordStatRow,
        r#"
        SELECT word, is_bigram as "is_bigram: bool", count
        FROM word_stats
        WHERE is_bigram = 0 AND scraped_at = ?
        ORDER BY count DESC
        LIMIT 50
        "#,
        latest
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let bigrams = sqlx::query_as!(
        WordStatRow,
        r#"
        SELECT word, is_bigram as "is_bigram: bool", count
        FROM word_stats
        WHERE is_bigram = 1 AND scraped_at = ?
        ORDER BY count DESC
        LIMIT 30
        "#,
        latest
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "words": words,
        "bigrams": bigrams,
        "scraped_at": latest.to_string(),
    })))
}
