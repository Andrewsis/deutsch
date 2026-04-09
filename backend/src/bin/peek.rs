use serde::Deserialize;
use sqlx::{MySqlPool, QueryBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct KaikkiForm {
    form: String,

    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct KaikkiWord {
    word: String,
    pos: String,
    forms: Option<Vec<KaikkiForm>>,
}

struct DbRecord {
    lemma: String,
    form: String,
    tags_json: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&db_url).await?;

    let file = File::open("kaikki.org-dictionary-German.jsonl").expect("no file");
    let reader = BufReader::new(file);

    let mut batch: Vec<DbRecord> = Vec::new();
    let batch_size = 3000;
    let mut total_inserted = 0;

    let start_time = Instant::now();

    for line_result in reader.lines() {
        let line = line_result?;

        if let Ok(entry) = serde_json::from_str::<KaikkiWord>(&line) {
            if entry.pos != "verb" {
                continue;
            }

            if let Some(forms) = entry.forms {
                for f in forms {
                    let tags_array = f.tags.unwrap_or_else(Vec::new);

                    let tags_json = serde_json::to_string(&tags_array).unwrap();

                    batch.push(DbRecord {
                        lemma: entry.word.clone(),
                        form: f.form,
                        tags_json,
                    });
                }
            }

            if batch.len() >= batch_size {
                insert_batch(&pool, &batch).await?;
                total_inserted += batch.len();
                batch.clear();
                println!("Вставлено форм: {}", total_inserted);
            }
        }
    }

    if !batch.is_empty() {
        insert_batch(&pool, &batch).await?;
        total_inserted += batch.len();
    }

    println!(
        "So many forms: {}, it took {:?}",
        total_inserted,
        start_time.elapsed()
    );
    Ok(())
}

async fn insert_batch(pool: &MySqlPool, items: &[DbRecord]) -> Result<(), sqlx::Error> {
    let mut query_builder = QueryBuilder::new("INSERT INTO verb_forms_kaikki (lemma, form, tags) ");

    query_builder.push_values(items, |mut b, item| {
        b.push_bind(&item.lemma);
        b.push_bind(&item.form);
        b.push_bind(&item.tags_json);
    });

    let query = query_builder.build();
    query.execute(pool).await?;

    Ok(())
}
