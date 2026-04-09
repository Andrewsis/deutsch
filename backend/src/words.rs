use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use sqlx::types::Json as SqlxJson;
use tracing::{error, info, warn};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct ExampleParams {
    pub word: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, FromRow)]
pub struct ExampleResponse {
    pub video_id: String,
    pub start_sec: f64,
    pub full_text: String,
}

pub async fn get_examples(
    State(state): State<AppState>,
    Query(params): Query<ExampleParams>,
) -> Result<Json<Vec<ExampleResponse>>, (StatusCode, String)> {
    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);

    let rows = sqlx::query_as::<_, ExampleResponse>(
        r#"
        SELECT
            video_id,
            CAST(start_sec AS DOUBLE) AS start_sec,
            text AS full_text
        FROM subtitles
        WHERE MATCH(text) AGAINST(CONCAT('"', ?, '"') IN BOOLEAN MODE)
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(&params.word)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await;

    match rows {
        Ok(examples) => {
            info!("Found {} examples for '{}'", examples.len(), params.word);
            Ok(Json(examples))
        }
        Err(e) => {
            error!("DB error searching examples for '{}': {}", params.word, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ))
        }
    }
}

#[derive(Serialize, FromRow)]
pub struct Word {
    pub lemma: String,
    pub aux: Option<String>,
    pub prs1sg: Option<String>,
    pub prs2sg: Option<String>,
    pub prs3sg: Option<String>,
    pub prs1pl: Option<String>,
    pub prs2pl: Option<String>,
    pub prs3pl: Option<String>,
    pub prt1sg: Option<String>,
    pub prt2sg: Option<String>,
    pub prt3sg: Option<String>,
    pub prt1pl: Option<String>,
    pub prt2pl: Option<String>,
    pub prt3pl: Option<String>,
    pub kj1_1sg: Option<String>,
    pub kj1_2sg: Option<String>,
    pub kj1_3sg: Option<String>,
    pub kj1_1pl: Option<String>,
    pub kj1_2pl: Option<String>,
    pub kj1_3pl: Option<String>,
    pub kj2_1sg: Option<String>,
    pub kj2_2sg: Option<String>,
    pub kj2_3sg: Option<String>,
    pub kj2_1pl: Option<String>,
    pub kj2_2pl: Option<String>,
    pub kj2_3pl: Option<String>,
    pub fut1_1sg: Option<String>,
    pub fut1_2sg: Option<String>,
    pub fut1_3sg: Option<String>,
    pub fut1_1pl: Option<String>,
    pub fut1_2pl: Option<String>,
    pub fut1_3pl: Option<String>,
    pub fut2_1sg: Option<String>,
    pub fut2_2sg: Option<String>,
    pub fut2_3sg: Option<String>,
    pub fut2_1pl: Option<String>,
    pub fut2_2pl: Option<String>,
    pub fut2_3pl: Option<String>,
    pub imp2sg: Option<String>,
    pub imp2pl: Option<String>,
    pub inf: Option<String>,
    pub pa1: Option<String>,
    pub pa2: Option<String>,
    pub translation_ua: Option<String>,
    pub praepositionen_ua: Option<SqlxJson<Value>>,
    pub audio_forms: Option<SqlxJson<Vec<String>>>,
}

pub async fn get_word(
    State(state): State<AppState>,
    Path(word): Path<String>,
) -> Result<Json<Word>, (StatusCode, String)> {
    let row = sqlx::query_as!(
        Word,
        r#"SELECT
        lemma, aux,
        prs1sg, prs2sg, prs3sg, prs1pl, prs2pl, prs3pl,
        prt1sg, prt2sg, prt3sg, prt1pl, prt2pl, prt3pl,
        kj1_1sg, kj1_2sg, kj1_3sg, kj1_1pl, kj1_2pl, kj1_3pl,
        kj2_1sg, kj2_2sg, kj2_3sg, kj2_1pl, kj2_2pl, kj2_3pl,
        fut1_1sg, fut1_2sg, fut1_3sg, fut1_1pl, fut1_2pl, fut1_3pl,
        fut2_1sg, fut2_2sg, fut2_3sg, fut2_1pl, fut2_2pl, fut2_3pl,
        imp2sg, imp2pl, inf, pa1, pa2,
        translation_ua,
        praepositionen_ua as "praepositionen_ua: SqlxJson<Value>",
        audio_forms as "audio_forms: SqlxJson<Vec<String>>"
        FROM verben_wide
        WHERE lemma = ?"#,
        word
    )
    .fetch_one(&state.pool)
    .await;

    match row {
        Ok(word_data) => {
            info!("Word '{}' found successfully", word);
            Ok(Json(word_data))
        }
        Err(sqlx::Error::RowNotFound) => {
            warn!("Word '{}' not found in database", word);
            Err((StatusCode::NOT_FOUND, "Word not found".to_string()))
        }
        Err(e) => {
            error!("DB error querying word '{}': {}", word, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ))
        }
    }
}
