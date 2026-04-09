use std::collections::HashMap;
use std::env;

use reqwest::Client;
use roxmltree::Document;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use tracing::{error, info, warn};

const STOP_WORDS: &[&str] = &[
    "der", "die", "das", "den", "dem", "des", "ein", "eine", "einen", "einem", "einer", "eines",
    "und", "oder", "aber", "doch", "sondern", "denn", "in", "im", "an", "am", "auf", "aus", "bei",
    "bis", "für", "gegen", "mit", "nach", "ohne", "seit", "um", "von", "vor", "während", "wegen",
    "zu", "zum", "zur", "über", "unter", "zwischen", "durch", "entlang", "ich", "du", "er", "sie",
    "es", "wir", "ihr", "mich", "dich", "sich", "uns", "euch", "mir", "dir", "ihm", "ihr", "uns",
    "euch", "ihnen", "mein", "dein", "sein", "unser", "euer", "dieser", "diese", "dieses",
    "diesen", "diesem", "jener", "jene", "jenes", "nicht", "kein", "keine", "keinen", "keinem",
    "als", "wie", "wenn", "weil", "dass", "ob", "wann", "wo", "was", "wer", "wie", "warum",
    "woher", "wohin", "auch", "noch", "schon", "nur", "sehr", "mehr", "so", "ja", "nein", "doch",
    "mal", "nun", "dann", "ist", "sind", "war", "waren", "wird", "werden", "hat", "haben", "hatte",
    "hatten", "habe", "sei", "wäre", "würde", "würden", "kann", "können", "muss", "müssen", "soll",
    "sollen", "will", "wollen", "darf", "dürfen", "mag", "mögen", "de", "www", "http", "https",
    "com",
];

struct RssSource {
    name: &'static str,
    url: &'static str,
}

const SOURCES: &[RssSource] = &[
    RssSource {
        name: "spiegel",
        url: "https://www.spiegel.de/schlagzeilen/index.rss",
    },
    RssSource {
        name: "zeit",
        url: "https://newsfeed.zeit.de/news/index",
    },
    RssSource {
        name: "tagesschau",
        url: "https://www.tagesschau.de/index~rss2.xml",
    },
    RssSource {
        name: "sueddeutsche",
        url: "https://rss.sueddeutsche.de/rss/Topthemen",
    },
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await?;

    info!("Running DB migration...");
    run_migration(&pool).await?;

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; GermanNewsScraper/1.0)")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let mut all_texts: Vec<String> = Vec::new();

    for source in SOURCES {
        info!("Fetching RSS from {}...", source.name);
        match fetch_rss(&client, source.url).await {
            Ok(texts) => {
                info!("  Got {} items from {}", texts.len(), source.name);
                all_texts.extend(texts);
            }
            Err(e) => {
                warn!("  Failed to fetch {}: {}", source.name, e);
            }
        }
    }

    if all_texts.is_empty() {
        error!("No texts fetched from any source");
        return Ok(());
    }

    info!("Total items collected: {}", all_texts.len());

    let combined = all_texts.join(" ");
    let tokens = tokenize(&combined);

    let word_counts = count_words(&tokens);
    let bigram_counts = count_bigrams(&tokens);

    info!(
        "Unique words: {}, unique bigrams: {}",
        word_counts.len(),
        bigram_counts.len()
    );

    save_to_db(&pool, &word_counts, &bigram_counts).await?;

    info!("Scraper finished successfully.");
    Ok(())
}

async fn run_migration(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS word_stats (
            id          INT AUTO_INCREMENT PRIMARY KEY,
            word        VARCHAR(255) NOT NULL,
            is_bigram   TINYINT(1)   NOT NULL DEFAULT 0,
            count       INT          NOT NULL DEFAULT 0,
            scraped_at  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
            INDEX idx_word (word),
            INDEX idx_scraped_at (scraped_at)
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn fetch_rss(client: &Client, url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let body = client.get(url).send().await?.text().await?;
    let doc = Document::parse(&body)?;

    let mut texts = Vec::new();

    for node in doc.descendants() {
        if node.is_element()
            && (node.tag_name().name() == "title" || node.tag_name().name() == "description")
        {
            if let Some(text) = node.text() {
                let clean = strip_html(text);
                if !clean.trim().is_empty() && clean.len() > 10 {
                    texts.push(clean);
                }
            }
        }
    }

    Ok(texts)
}

fn strip_html(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphabetic())
        .map(|w| w.to_lowercase())
        .filter(|w| {
            w.len() >= 3
                && !STOP_WORDS.contains(&w.as_str())
                && w.chars().all(|c| c.is_alphabetic())
        })
        .collect()
}

fn count_words(tokens: &[String]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in tokens {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }
    counts.retain(|_, v| *v >= 2);
    counts
}

fn count_bigrams(tokens: &[String]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for window in tokens.windows(2) {
        let bigram = format!("{} {}", window[0], window[1]);
        *counts.entry(bigram).or_insert(0) += 1;
    }
    counts.retain(|_, v| *v >= 2);
    counts
}

async fn save_to_db(
    pool: &MySqlPool,
    words: &HashMap<String, u32>,
    bigrams: &HashMap<String, u32>,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();

    let mut word_vec: Vec<(&String, &u32)> = words.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));
    word_vec.truncate(200);

    for (word, count) in &word_vec {
        sqlx::query!(
            "INSERT INTO word_stats (word, is_bigram, count, scraped_at) VALUES (?, 0, ?, ?)",
            word,
            *count,
            now
        )
        .execute(pool)
        .await?;
    }
    info!("Saved {} words", word_vec.len());

    let mut bigram_vec: Vec<(&String, &u32)> = bigrams.iter().collect();
    bigram_vec.sort_by(|a, b| b.1.cmp(a.1));
    bigram_vec.truncate(100);

    for (bigram, count) in &bigram_vec {
        sqlx::query!(
            "INSERT INTO word_stats (word, is_bigram, count, scraped_at) VALUES (?, 1, ?, ?)",
            bigram,
            *count,
            now
        )
        .execute(pool)
        .await?;
    }
    info!("Saved {} bigrams", bigram_vec.len());

    Ok(())
}
