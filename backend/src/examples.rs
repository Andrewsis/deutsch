//! Example sentences for a verb (Tatoeba / Wiktionary, loaded by `bin/load_examples.rs`)
//! and lemma lookup by an inflected form.

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::lang;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct SentenceParams {
    pub verb: String,
    pub lang: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize)]
pub struct Sentence {
    pub text: String,
    /// Space-separated words of the sentence to highlight (the verb form, e.g. "wartet ab").
    pub form: String,
    /// Translation into the requested language, if there is one (no fallback to another language).
    pub translation: Option<String>,
    pub source: String,
    pub source_url: String,
    pub author: Option<String>,
    /// Bibliographic reference of a Wiktionary quotation.
    pub citation: Option<String>,
    /// Meaning of the verb this example illustrates (Wiktionary gloss).
    pub sense: Option<String>,
}

fn db_error(e: sqlx::Error) -> (StatusCode, String) {
    error!("DB error: {}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
}

fn source_url(source: &str, source_ref: &str) -> String {
    let page = source_ref.replace(' ', "_");
    match source {
        "tatoeba" => format!("https://tatoeba.org/sentences/show/{source_ref}"),
        "wiktionary_en" => format!("https://en.wiktionary.org/wiki/{page}#German"),
        _ => format!("https://de.wiktionary.org/wiki/{page}"),
    }
}

/// `GET /api/sentences?verb=&lang=&limit=&offset=`
///
/// Order: translated into `lang` first, then short self-made examples before literary
/// quotations, then sentences of 4..=14 words, then the ones without rare words,
/// Wiktionary before Tatoeba.
pub async fn get_sentences(
    State(state): State<AppState>,
    Query(params): Query<SentenceParams>,
) -> Result<Json<Vec<Sentence>>, (StatusCode, String)> {
    let lang = lang::resolve(params.lang.as_deref());
    let limit = params.limit.unwrap_or(10).min(50);
    let offset = params.offset.unwrap_or(0);

    let rows = sqlx::query!(
        r#"SELECT e.text, ve.form, t.text AS "translation?", e.source, e.source_ref,
                  e.author, e.citation, e.sense
           FROM verben_wide v
           JOIN verb_examples ve ON ve.verb_id = v.id
           JOIN example_sentences e ON e.id = ve.example_id
           LEFT JOIN example_translations t ON t.example_id = e.id AND t.lang = ?
           WHERE v.lemma = ?
           ORDER BY t.text IS NULL,
                    e.citation IS NOT NULL,
                    e.word_count NOT BETWEEN 4 AND 14,
                    e.rarity DIV 20000,
                    e.source = 'tatoeba',
                    e.rarity,
                    e.id
           LIMIT ? OFFSET ?"#,
        lang,
        params.verb,
        limit,
        offset
    )
    .fetch_all(&state.pool)
    .await
    .map_err(db_error)?;

    Ok(Json(
        rows.into_iter()
            .map(|r| Sentence {
                source_url: source_url(&r.source, &r.source_ref),
                text: r.text,
                form: r.form,
                translation: r.translation,
                source: r.source,
                author: r.author,
                citation: r.citation,
                sense: r.sense,
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
pub struct LemmaParams {
    pub form: String,
}

/// `GET /api/lemma?form=wartete` -> `["warten"]`: verbs from `verben_wide` that have this
/// form (Morphy `word_forms`), most frequent first. Used to redirect a search for an
/// inflected form to the verb page.
pub async fn get_lemma(
    State(state): State<AppState>,
    Query(params): Query<LemmaParams>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let form = params.form.trim();
    let rows = sqlx::query_scalar!(
        r#"SELECT v.lemma
           FROM word_forms f
           JOIN verben_wide v ON v.lemma = f.lemma COLLATE utf8mb4_bin
           LEFT JOIN lemma_frequency lf ON lf.lemma = f.lemma COLLATE utf8mb4_bin
           WHERE f.pos = 'VER' AND (f.form = ? OR f.form = LOWER(?))
           GROUP BY v.lemma
           ORDER BY MIN(COALESCE(lf.freq_class, 99)), v.lemma
           LIMIT 5"#,
        form,
        form
    )
    .fetch_all(&state.pool)
    .await
    .map_err(db_error)?;
    Ok(Json(rows))
}
