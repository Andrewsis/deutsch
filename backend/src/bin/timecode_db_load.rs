use serde::Deserialize;
use sqlx::{QueryBuilder, mysql::MySqlPoolOptions};
use std::time::Instant;
use std::{env, fs};

#[derive(Deserialize, Debug)]
struct SubtitleJson {
    text: String,
    start_sec: f64,
}

#[derive(Deserialize, Debug)]
struct VideoJson {
    video_id: String,
    views: Option<u64>,
    upload_date: Option<String>,
    subtitles: Option<Vec<SubtitleJson>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let start_time = Instant::now();

    println!("connecting to db");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let file_content =
        fs::read_to_string("database_ready.json").expect("coulndt read database_ready.json");

    let videos: Vec<VideoJson> = serde_json::from_str(&file_content).expect("couldnt prase");

    println!("found so many videos to download: {}", videos.len());

    let mut videos_added = 0;
    let mut subs_added = 0;

    for video in videos {
        let year: u32 = video
            .upload_date
            .as_deref()
            .unwrap_or("1970")
            .chars()
            .take(4)
            .collect::<String>()
            .parse()
            .unwrap_or(1970);

        let views = video.views.unwrap_or(0);

        let insert_result =
            sqlx::query("INSERT IGNORE INTO videos (id, views, upload_year) VALUES (?, ?, ?)")
                .bind(&video.video_id)
                .bind(views)
                .bind(year)
                .execute(&pool)
                .await?;

        if insert_result.rows_affected() == 0 {
            println!("skip video {} already in db.", video.video_id);
            continue;
        }

        videos_added += 1;

        if let Some(subs) = video.subtitles {
            if subs.is_empty() {
                continue;
            }

            for chunk in subs.chunks(1000) {
                let mut query_builder =
                    QueryBuilder::new("INSERT INTO subtitles (video_id, start_sec, text) ");

                query_builder.push_values(chunk, |mut b, sub| {
                    b.push_bind(&video.video_id)
                        .push_bind(sub.start_sec)
                        .push_bind(&sub.text);
                });

                let query = query_builder.build();
                query.execute(&pool).await?;
            }

            subs_added += subs.len();
            println!(
                "added videos {} (subtitles: {})",
                video.video_id,
                subs.len()
            );
        }
    }

    println!();
    println!();
    println!("FINISH");
    println!("new videos: {}", videos_added);
    println!("new subtitle lines: {}", subs_added);
    println!("it took: {:?}", start_time.elapsed());

    Ok(())
}
