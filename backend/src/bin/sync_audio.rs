use sqlx::{Row, mysql::MySqlPoolOptions};
use std::collections::HashSet;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("Начинаем синхронизацию аудио-форм...");
    let start_time = Instant::now();

    let words = sqlx::query(
        "SELECT id, lemma, prs1sg, prs2sg, prs3sg, prs1pl, prs2pl, prs3pl,
                prt1sg, prt2sg, prt3sg, prt1pl, prt2pl, prt3pl,
                pa1, pa2, imp2sg, imp2pl, inf
         FROM verben_wide",
    )
    .fetch_all(&pool)
    .await?;

    let mut words_updated = 0;

    for row in words {
        let id: i32 = row.get("id");

        let mut all_forms = HashSet::new();

        let columns = vec![
            "lemma", "prs1sg", "prs2sg", "prs3sg", "prs1pl", "prs2pl", "prs3pl", "prt1sg",
            "prt2sg", "prt3sg", "prt1pl", "prt2pl", "prt3pl", "pa1", "pa2", "imp2sg", "imp2pl",
            "inf",
        ];

        for col in columns {
            if let Ok(form) = row.try_get::<String, _>(col) {
                if !form.trim().is_empty() {
                    all_forms.insert(form);
                }
            }
        }

        let mut found_audio_forms = Vec::new();

        for form in all_forms {
            let exists: bool = sqlx::query_scalar(
                r#"SELECT EXISTS(
                    SELECT 1 FROM subtitles
                    WHERE MATCH(text) AGAINST(CONCAT('"', ?, '"') IN BOOLEAN MODE)
                )"#,
            )
            .bind(&form)
            .fetch_one(&pool)
            .await?;

            if exists {
                found_audio_forms.push(form);
            }
        }

        let json_forms = serde_json::to_string(&found_audio_forms)?;

        sqlx::query("UPDATE verben_wide SET audio_forms = ? WHERE id = ?")
            .bind(json_forms)
            .bind(id)
            .execute(&pool)
            .await?;

        words_updated += 1;

        if words_updated % 50 == 0 {
            println!("words checked: {}", words_updated);
        }
    }

    println!("sync successfully!");
    println!("updated words: {}", words_updated);
    println!("it took: {:?}", start_time.elapsed());

    Ok(())
}
