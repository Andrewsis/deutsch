use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use crate::auth::Claims;
use crate::state::AppState;

#[derive(Deserialize, Debug)]
pub struct ReviewRequest {
    pub verb_id: i32,
    pub quality: u8,
}

#[derive(Serialize)]
pub struct FlashcardResponse {
    pub id: i32,
    pub lemma: String,
    pub translation_ua: Option<String>,
    pub prs3sg: Option<String>,
    pub prt3sg: Option<String>,
    pub pa2: Option<String>,
    pub aux: Option<String>,
}

#[instrument(skip(state))]
pub async fn get_due_flashcards(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<FlashcardResponse>>, (StatusCode, String)> {
    let username = claims.sub;
    info!("Fetching due flashcards for user: {}", username);

    let words = sqlx::query_as!(
        FlashcardResponse,
        r#"
        SELECT v.id, v.lemma, v.translation_ua,
               v.prs3sg, v.prt3sg, v.pa2, v.aux
        FROM verben_wide v
        ORDER BY RAND()
        LIMIT 10
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error fetching flashcards: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
    })?;

    Ok(Json(words))
}

#[instrument(skip(state))]
pub async fn review_flashcard(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<ReviewRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = sqlx::query!("SELECT id FROM users WHERE username = ?", claims.sub)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".into()))?;

    let card = sqlx::query!(
        "SELECT repetition, interval_days, ease_factor FROM user_flashcards WHERE user_id = ? AND verb_id = ?",
        user.id,
        payload.verb_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
    })?;

    let (rep, int, ease) = match card {
        Some(c) => (
            c.repetition.unwrap_or(0),
            c.interval_days.unwrap_or(0),
            c.ease_factor.unwrap_or(2.5),
        ),
        None => (0, 0, 2.5),
    };

    let (new_rep, new_int, new_ease) = calculate_sm2(payload.quality, rep, int, ease);

    sqlx::query!(
        r#"
        INSERT INTO user_flashcards (user_id, verb_id, repetition, interval_days, ease_factor, next_review)
        VALUES (?, ?, ?, ?, ?, DATE_ADD(NOW(), INTERVAL ? DAY))
        ON DUPLICATE KEY UPDATE
            repetition = VALUES(repetition),
            interval_days = VALUES(interval_days),
            ease_factor = VALUES(ease_factor),
            next_review = VALUES(next_review)
        "#,
        user.id, payload.verb_id, new_rep, new_int, new_ease, new_int
    )
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!("Failed to save review: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
    })?;

    Ok(StatusCode::OK)
}

pub fn calculate_sm2(
    quality: u8,
    mut repetition: i32,
    mut interval: i32,
    mut ease_factor: f32,
) -> (i32, i32, f32) {
    if quality == 0 {
        repetition = 0;
        interval = 1;
    } else {
        if repetition == 0 {
            interval = 1;
        } else if repetition == 1 {
            interval = 6;
        } else {
            interval = (interval as f32 * ease_factor).round() as i32;
        }
        repetition += 1;
    }

    ease_factor += 0.1 - (3.0 - quality as f32) * (0.08 + (3.0 - quality as f32) * 0.02);
    if ease_factor < 1.3 {
        ease_factor = 1.3;
    }

    (repetition, interval, ease_factor)
}
