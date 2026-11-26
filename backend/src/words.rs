use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use sqlx::types::Json as SqlxJson;
use tracing::{error, info, warn};

use crate::lang::{self, LangQuery};
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
    /// Start of the next subtitle line in the same video (end of this phrase), if any.
    pub end_sec: Option<f64>,
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
            s.video_id,
            CAST(s.start_sec AS DOUBLE) AS start_sec,
            CAST((
                SELECT MIN(n.start_sec)
                FROM subtitles n
                WHERE n.video_id = s.video_id AND n.start_sec > s.start_sec
            ) AS DOUBLE) AS end_sec,
            s.text AS full_text
        FROM subtitles s
        WHERE MATCH(s.text) AGAINST(CONCAT('"', ?, '"') IN BOOLEAN MODE)
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
    pub translation: Option<String>,
    pub prepositions: Option<SqlxJson<Value>>,
    pub audio_forms: Option<SqlxJson<Vec<String>>>,
}

pub async fn get_word(
    State(state): State<AppState>,
    Path(word): Path<String>,
    Query(lang): Query<LangQuery>,
) -> Result<Json<Word>, (StatusCode, String)> {
    let row = sqlx::query_as!(
        Word,
        r#"SELECT
        v.lemma, v.aux,
        v.prs1sg, v.prs2sg, v.prs3sg, v.prs1pl, v.prs2pl, v.prs3pl,
        v.prt1sg, v.prt2sg, v.prt3sg, v.prt1pl, v.prt2pl, v.prt3pl,
        v.kj1_1sg, v.kj1_2sg, v.kj1_3sg, v.kj1_1pl, v.kj1_2pl, v.kj1_3pl,
        v.kj2_1sg, v.kj2_2sg, v.kj2_3sg, v.kj2_1pl, v.kj2_2pl, v.kj2_3pl,
        v.fut1_1sg, v.fut1_2sg, v.fut1_3sg, v.fut1_1pl, v.fut1_2pl, v.fut1_3pl,
        v.fut2_1sg, v.fut2_2sg, v.fut2_3sg, v.fut2_1pl, v.fut2_2pl, v.fut2_3pl,
        v.imp2sg, v.imp2pl, v.inf, v.pa1, v.pa2,
        COALESCE(t.translation, d.translation) AS translation,
        COALESCE(t.prepositions, d.prepositions) as "prepositions: SqlxJson<Value>",
        v.audio_forms as "audio_forms: SqlxJson<Vec<String>>"
        FROM verben_wide v
        LEFT JOIN verb_translations t ON t.verb_id = v.id AND t.lang = ?
        LEFT JOIN verb_translations d ON d.verb_id = v.id AND d.lang = ?
        WHERE v.lemma = ?"#,
        lang.code(),
        lang::DEFAULT,
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
