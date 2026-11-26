use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use rand::rng;
use rand::seq::SliceRandom;
use sqlx::MySqlPool;
use tracing::{error, info};

/// One quiz question. `options` / `correct` are indexed by player (0 = player 1, 1 = player 2),
/// because each player sees translations in their own language.
#[derive(Clone)]
pub struct Question {
    pub index: usize,
    pub lemma: String,
    pub options: [Vec<String>; 2],
    pub correct: [String; 2],
}

impl Question {
    fn for_client(&self, player: usize) -> serde_json::Value {
        serde_json::json!({ "index": self.index, "lemma": self.lemma, "options": self.options[player] })
    }
}

async fn translation_for(pool: &MySqlPool, verb_id: i32, lang: &str) -> Option<String> {
    sqlx::query_scalar!(
        r#"
        SELECT COALESCE(t.translation, d.translation) AS "translation?"
        FROM verben_wide v
        LEFT JOIN verb_translations t ON t.verb_id = v.id AND t.lang = ?
        LEFT JOIN verb_translations d ON d.verb_id = v.id AND d.lang = ?
        WHERE v.id = ?
        "#,
        lang,
        crate::lang::DEFAULT,
        verb_id
    )
    .fetch_one(pool)
    .await
    .ok()
    .flatten()
}

pub async fn build_questions(pool: &MySqlPool, langs: [&str; 2]) -> Option<Vec<Question>> {
    let words = sqlx::query!(
        "
        SELECT v.id, v.lemma
        FROM verben_wide v
        JOIN verb_translations t ON t.verb_id = v.id AND t.lang = ?
        WHERE t.translation IS NOT NULL
        ORDER BY RAND()
        LIMIT 5
        ",
        crate::lang::DEFAULT
    )
    .fetch_all(pool)
    .await
    .ok()?;

    let mut questions = Vec::new();

    for (i, word) in words.iter().enumerate() {
        let distractors = sqlx::query_scalar!(
            "
            SELECT verb_id
            FROM verb_translations
            WHERE lang = ? AND translation IS NOT NULL AND verb_id != ?
            ORDER BY RAND()
            LIMIT 3
            ",
            crate::lang::DEFAULT,
            word.id
        )
        .fetch_all(pool)
        .await
        .ok()?;

        let mut ids = distractors;
        ids.push(word.id);
        ids.shuffle(&mut rng());

        let mut options: [Vec<String>; 2] = [Vec::new(), Vec::new()];
        let mut correct: [String; 2] = [String::new(), String::new()];
        for (player, lang) in langs.iter().enumerate() {
            for &id in &ids {
                let text = translation_for(pool, id, lang).await.unwrap_or_default();
                if id == word.id {
                    correct[player] = text.clone();
                }
                options[player].push(text);
            }
        }

        questions.push(Question {
            index: i,
            lemma: word.lemma.clone(),
            options,
            correct,
        });
    }

    Some(questions)
}

pub async fn run_game(
    mut socket1: WebSocket,
    user1: String,
    mut socket2: WebSocket,
    user2: String,
    langs: [&'static str; 2],
    pool: MySqlPool,
) {
    info!("Match starting: {} vs {}", user1, user2);

    let questions = match build_questions(&pool, langs).await {
        Some(q) => q,
        None => {
            error!("Failed to build questions");
            return;
        }
    };

    let questions_for = |player: usize| -> Vec<serde_json::Value> {
        questions.iter().map(|q| q.for_client(player)).collect()
    };

    let _ = socket1
        .send(Message::Text(
            serde_json::json!({
                "action": "start", "opponent": user2, "questions": questions_for(0),
            })
            .to_string()
            .into(),
        ))
        .await;

    let _ = socket2
        .send(Message::Text(
            serde_json::json!({
                "action": "start", "opponent": user1, "questions": questions_for(1),
            })
            .to_string()
            .into(),
        ))
        .await;

    let mut score1: u32 = 0;
    let mut score2: u32 = 0;
    let mut answers1: u32 = 0;
    let mut answers2: u32 = 0;
    let total = questions.len() as u32;

    loop {
        if answers1 >= total && answers2 >= total {
            break;
        }

        tokio::select! {
            msg = socket1.next(), if answers1 < total => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(text.as_str()) {
                            if let (Some(idx), Some(ans)) = (val["index"].as_u64(), val["answer"].as_str()) {
                                answers1 += 1;
                                if let Some(question) = questions.get(idx as usize) {
                                    if ans == question.correct[0] { score1 += 1; }
                                }
                                let _ = socket1.send(Message::Text(serde_json::json!({
                                    "action": "score_update",
                                    "your_score": score1,
                                    "answered": answers1,
                                }).to_string().into())).await;
                            }
                        }
                    }
                    None | Some(Err(_)) => {
                        let _ = socket2.send(Message::Text(serde_json::json!({
                            "action": "finish", "result": "win",
                            "reason": "opponent_disconnected",
                            "your_score": score2, "opponent_score": score1,
                        }).to_string().into())).await;
                        return;
                    }
                    _ => {}
                }
            }
            msg = socket2.next(), if answers2 < total => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(text.as_str()) {
                            if let (Some(idx), Some(ans)) = (val["index"].as_u64(), val["answer"].as_str()) {
                                answers2 += 1;
                                if let Some(q) = questions.get(idx as usize) {
                                    if ans == q.correct[1] { score2 += 1; }
                                }
                                let _ = socket2.send(Message::Text(serde_json::json!({
                                    "action": "score_update",
                                    "your_score": score2,
                                    "answered": answers2,
                                }).to_string().into())).await;
                            }
                        }
                    }
                    None | Some(Err(_)) => {
                        let _ = socket1.send(Message::Text(serde_json::json!({
                            "action": "finish", "result": "win",
                            "reason": "opponent_disconnected",
                            "your_score": score1, "opponent_score": score2,
                        }).to_string().into())).await;
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    let result1 = if score1 > score2 {
        "win"
    } else if score1 < score2 {
        "lose"
    } else {
        "draw"
    };
    let result2 = if score2 > score1 {
        "win"
    } else if score2 < score1 {
        "lose"
    } else {
        "draw"
    };

    let _ = socket1
        .send(Message::Text(
            serde_json::json!({
                "action": "finish", "result": result1,
                "your_score": score1, "opponent_score": score2, "opponent": user2,
            })
            .to_string()
            .into(),
        ))
        .await;

    let _ = socket2
        .send(Message::Text(
            serde_json::json!({
                "action": "finish", "result": result2,
                "your_score": score2, "opponent_score": score1, "opponent": user1,
            })
            .to_string()
            .into(),
        ))
        .await;

    info!(
        "Match finished: {} ({}) vs {} ({})",
        user1, score1, user2, score2
    );
}
