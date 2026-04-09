use serde::Deserialize;
use sqlx::{MySqlPool, QueryBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct KaikkiVerb {
    word: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&db_url).await?;

    let file = File::open("german_verbs.jsonl").expect("no file");

    let reader = BufReader::new(file);

    let mut batch = Vec::new();
    let batch_size = 2000;
    let mut total_inserted = 0;

    let start_time = Instant::now();

    for line in reader.lines() {
        let line = line?;

        if let Ok(entry) = serde_json::from_str::<KaikkiVerb>(&line) {
            batch.push(entry);

            if batch.len() >= batch_size {
                insert_batch(&pool, &batch).await?;
                total_inserted += batch.len();
                batch.clear();
                println!("Inserted: {}", total_inserted);
            }
        }
    }

    if !batch.is_empty() {
        insert_batch(&pool, &batch).await?;
        total_inserted += batch.len();
    }

    println!(
        "added: {} it took: {:?}",
        total_inserted,
        start_time.elapsed()
    );
    Ok(())
}

async fn insert_batch(pool: &MySqlPool, items: &[KaikkiVerb]) -> Result<(), sqlx::Error> {
    let mut query_builder = QueryBuilder::new("INSERT INTO verb_meta (lemma) ");

    query_builder.push_values(items, |mut b, item| {
        b.push_bind(&item.word);
    });

    query_builder.push(" ON DUPLICATE KEY UPDATE lemma=lemma");

    let query = query_builder.build();
    query.execute(pool).await?;

    Ok(())
}
