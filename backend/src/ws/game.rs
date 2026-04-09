use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use rand::rng;
use rand::seq::SliceRandom;
use serde::Serialize;
use sqlx::MySqlPool;
use tracing::{error, info};

#[derive(Serialize, Clone)]
pub struct Question {
    pub index: usize,
    pub lemma: String,
    pub options: Vec<String>,
    pub correct: String,
}

pub async fn build_questions(pool: &MySqlPool) -> Option<Vec<Question>> {
    let words = sqlx::query!(
        "SELECT lemma, translation_ua FROM verben_wide WHERE translation_ua IS NOT NULL ORDER BY RAND() LIMIT 5"
    )
    .fetch_all(pool)
    .await
    .ok()?;

    let mut questions = Vec::new();

    for (i, word) in words.iter().enumerate() {
        let correct = word.translation_ua.clone().unwrap_or_default();

        let distractors = sqlx::query!(
            "SELECT translation_ua FROM verben_wide WHERE translation_ua IS NOT NULL AND lemma != ? ORDER BY RAND() LIMIT 3",
            word.lemma
        )
        .fetch_all(pool)
        .await
        .ok()?;

        let mut options: Vec<String> = distractors
            .into_iter()
            .map(|r| r.translation_ua.unwrap_or_default())
            .collect();

        options.push(correct.clone());
        options.shuffle(&mut rng());

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
    pool: MySqlPool,
) {
    info!("Match starting: {} vs {}", user1, user2);

    let questions = match build_questions(&pool).await {
        Some(q) => q,
        None => {
            error!("Failed to build questions");
            return;
        }
    };

    let questions_for_client: Vec<serde_json::Value> = questions
        .iter()
        .map(|q| serde_json::json!({ "index": q.index, "lemma": q.lemma, "options": q.options }))
        .collect();

    let _ = socket1
        .send(Message::Text(
            serde_json::json!({
                "action": "start", "opponent": user2, "questions": questions_for_client,
            })
            .to_string()
            .into(),
        ))
        .await;

    let _ = socket2
        .send(Message::Text(
            serde_json::json!({
                "action": "start", "opponent": user1, "questions": questions_for_client,
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
                                if let Some(q) = questions.get(idx as usize) {
                                    if ans == q.correct { score1 += 1; }
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
                                    if ans == q.correct { score2 += 1; }
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
