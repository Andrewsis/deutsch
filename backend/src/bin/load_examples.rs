//! Loads German example sentences for verbs into `example_sentences`, `example_translations`
//! and `verb_examples`.
//!
//! Sources (downloaded by `tools/fetch_data.sh` into `data/raw/`):
//! - `tatoeba`: deu_sentences_detailed.tsv + deu-{ukr,eng}_links.tsv + {ukr,eng}_sentences.tsv
//! - `wiktionary-en`: kaikki.org-dictionary-German-by-pos-verb.jsonl (English Wiktionary, with English translations)
//! - `wiktionary-de`: dewiktionary.jsonl.gz (German Wiktionary, no translations)
//!
//! Usage (from backend/): `cargo run --release --bin load_examples -- <tatoeba|wiktionary-en|wiktionary-de|all> [data_dir]`
//! Every run replaces all rows of the given source, so it can be re-run safely.
//!
//! Needs `word_forms` (Morphy) and `form_frequency` (DeReKo) to be loaded, see db/migrations/002.

use flate2::read::GzDecoder;
use serde_json::Value;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool, QueryBuilder, Row};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

type BoxError = Box<dyn std::error::Error>;

/// Sentences with more words are not stored (long literary quotations are useless as examples).
const MAX_WORDS: usize = 40;
/// Rank given to words outside the DeReKo top 100k.
const RARE_RANK: u32 = 100_001;
const BATCH: usize = 1000;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenvy::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let what = args.get(1).map(String::as_str).unwrap_or("");
    let dir = PathBuf::from(args.get(2).map(String::as_str).unwrap_or("../data/raw"));
    if !["tatoeba", "wiktionary-en", "wiktionary-de", "all"].contains(&what) {
        eprintln!("usage: load_examples <tatoeba|wiktionary-en|wiktionary-de|all> [data_dir]");
        std::process::exit(2);
    }

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new().max_connections(2).connect(&db_url).await?;

    let t = Instant::now();
    let lex = Lexicon::load(&pool).await?;
    println!(
        "lexicon: {} verbs, {} forms, {} separable stems, {} ranked forms ({:?})",
        lex.verbs.len(),
        lex.forms.len(),
        lex.separable.len(),
        lex.ranks.len(),
        t.elapsed()
    );

    if what == "tatoeba" || what == "all" {
        let exs = read_tatoeba(&dir, &lex)?;
        write_source(&pool, "tatoeba", exs).await?;
    }
    if what == "wiktionary-en" || what == "all" {
        let f = File::open(dir.join("kaikki.org-dictionary-German-by-pos-verb.jsonl"))?;
        let exs = read_wiktionary(BufReader::new(f), &lex, "wiktionary_en")?;
        write_source(&pool, "wiktionary_en", exs).await?;
    }
    if what == "wiktionary-de" || what == "all" {
        let f = File::open(dir.join("dewiktionary.jsonl.gz"))?;
        let exs = read_wiktionary(BufReader::new(GzDecoder::new(f)), &lex, "wiktionary_de")?;
        write_source(&pool, "wiktionary_de", exs).await?;
    }

    println!("done in {:?}", t.elapsed());
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Lexicon and matching

struct Lexicon {
    /// verben_wide lemma -> id
    verbs: HashMap<String, i32>,
    /// single-word verb form -> verb ids (Morphy word_forms, pos = VER)
    forms: HashMap<String, Vec<i32>>,
    /// finite stem of a separable verb -> (prefix, verb id), from verben_wide values like "wartet ab"
    separable: HashMap<String, Vec<(String, i32)>>,
    /// word form -> DeReKo frequency rank
    ranks: HashMap<String, u32>,
}

impl Lexicon {
    async fn load(pool: &MySqlPool) -> Result<Self, BoxError> {
        let mut verbs = HashMap::new();
        let mut separable: HashMap<String, Vec<(String, i32)>> = HashMap::new();
        let mut forms: HashMap<String, Vec<i32>> = HashMap::new();

        let rows = sqlx::query(
            "SELECT id, lemma, prs1sg, prs2sg, prs3sg, prs1pl, prs2pl, prs3pl,
                    prt1sg, prt2sg, prt3sg, prt1pl, prt2pl, prt3pl, imp2sg, imp2pl, pa2
             FROM verben_wide",
        )
        .fetch_all(pool)
        .await?;
        for row in &rows {
            let id: i32 = row.get("id");
            let lemma: String = row.get("lemma");
            for col in 2..row.len() {
                let Ok(Some(value)) = row.try_get::<Option<String>, _>(col) else { continue };
                let parts: Vec<&str> = value.split_whitespace().collect();
                match parts.as_slice() {
                    [single] => push_unique(forms.entry(single.to_string()).or_default(), id),
                    [stem, prefix] if lemma.starts_with(prefix) => {
                        let list = separable.entry(stem.to_string()).or_default();
                        if !list.iter().any(|(p, v)| p == prefix && *v == id) {
                            list.push((prefix.to_string(), id));
                        }
                    }
                    _ => {}
                }
            }
            push_unique(forms.entry(lemma.clone()).or_default(), id);
            verbs.insert(lemma, id);
        }

        let rows = sqlx::query(
            "SELECT DISTINCT f.form, v.id
             FROM word_forms f
             JOIN verben_wide v ON v.lemma COLLATE utf8mb4_bin = f.lemma
             WHERE f.pos = 'VER'",
        )
        .fetch_all(pool)
        .await?;
        for row in &rows {
            let form: String = row.get(0);
            push_unique(forms.entry(form).or_default(), row.get(1));
        }

        let mut ranks = HashMap::new();
        for row in sqlx::query("SELECT form, `rank` FROM form_frequency ORDER BY `rank`")
            .fetch_all(pool)
            .await?
        {
            let rank: i32 = row.get(1);
            ranks.entry(row.get::<String, _>(0)).or_insert(rank as u32);
        }

        Ok(Self { verbs, forms, separable, ranks })
    }

    /// Finds verbs in a sentence. Returns (verb id, space-separated matched words).
    /// A separable verb counts only if its prefix stands later at the end of a clause
    /// ("Wir warten ab." -> abwarten, "Ich warte auf dich." -> warten).
    fn match_verbs(&self, toks: &[Tok]) -> Vec<(i32, String)> {
        let mut found: Vec<(i32, Vec<String>)> = Vec::new();
        let mut add = |id: i32, words: &[&str]| {
            let entry = match found.iter_mut().position(|(v, _)| *v == id) {
                Some(i) => &mut found[i].1,
                None => {
                    found.push((id, Vec::new()));
                    &mut found.last_mut().unwrap().1
                }
            };
            for w in words {
                if !entry.iter().any(|e| e == w) {
                    entry.push(w.to_string());
                }
            }
        };

        for (i, tok) in toks.iter().enumerate() {
            let lowered = (i == 0 || tok.after_quote).then(|| decapitalize(tok.text)).flatten();
            let variants = std::iter::once(tok.text).chain(lowered.as_deref());

            for word in variants {
                let mut separated = false;
                if let Some(list) = self.separable.get(word) {
                    for (prefix, id) in list {
                        if let Some(p) = toks[i + 1..]
                            .iter()
                            .find(|t| t.text == prefix && t.clause_final)
                        {
                            add(*id, &[tok.text, p.text]);
                            separated = true;
                        }
                    }
                }
                if !separated && let Some(ids) = self.forms.get(word) {
                    for id in ids {
                        add(*id, &[tok.text]);
                    }
                }
            }
        }
        found.into_iter().map(|(id, w)| (id, w.join(" "))).collect()
    }

    /// Largest frequency rank among the words of the sentence (how hard the rarest word is).
    fn rarity(&self, toks: &[Tok]) -> u32 {
        toks.iter()
            .map(|t| {
                self.ranks
                    .get(t.text)
                    .or_else(|| decapitalize(t.text).and_then(|l| self.ranks.get(&l)))
                    .copied()
                    .unwrap_or(RARE_RANK)
            })
            .max()
            .unwrap_or(0)
    }
}

fn push_unique(v: &mut Vec<i32>, id: i32) {
    if !v.contains(&id) {
        v.push(id);
    }
}

fn decapitalize(w: &str) -> Option<String> {
    let mut chars = w.chars();
    let first = chars.next()?;
    first
        .is_uppercase()
        .then(|| first.to_lowercase().chain(chars).collect())
}

struct Tok<'a> {
    text: &'a str,
    /// next non-space character is punctuation or the end of the sentence
    clause_final: bool,
    /// preceded by an opening quote or colon (so it may be capitalized as a sentence start)
    after_quote: bool,
}

fn tokenize(s: &str) -> Vec<Tok<'_>> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let mut spans = Vec::new();
    for (i, c) in s.char_indices() {
        if c.is_alphabetic() {
            start.get_or_insert(i);
        } else if let Some(st) = start.take() {
            spans.push((st, i));
        }
    }
    if let Some(st) = start {
        spans.push((st, s.len()));
    }
    for (st, end) in spans {
        let next = s[end..].trim_start().chars().next();
        let prev = s[..st].trim_end().chars().last();
        out.push(Tok {
            text: &s[st..end],
            clause_final: next.is_none_or(|c| ".,!?;:\"“”„»«)–—-…".contains(c)),
            after_quote: prev.is_some_and(|c| "\"„“»«:".contains(c)),
        });
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Sources

struct Example {
    source_ref: String,
    text: String,
    author: Option<String>,
    citation: Option<String>,
    sense: Option<String>,
    word_count: usize,
    rarity: u32,
    /// (lang, text, origin)
    translations: Vec<(&'static str, String, &'static str)>,
    /// (verb id, words to highlight)
    links: Vec<(i32, String)>,
}

fn read_tsv(path: &Path, mut f: impl FnMut(Vec<&str>)) -> Result<(), BoxError> {
    let reader = BufReader::new(File::open(path)?);
    for line in reader.lines() {
        let line = line?;
        f(line.split('\t').collect());
    }
    Ok(())
}

/// deu id -> first linked sentence id in the other language
fn read_links(path: &Path) -> Result<HashMap<u32, u32>, BoxError> {
    let mut links = HashMap::new();
    read_tsv(path, |c| {
        if let (Some(Ok(a)), Some(Ok(b))) = (c.first().map(|s| s.parse()), c.get(1).map(|s| s.parse())) {
            links.entry(a).or_insert(b);
        }
    })?;
    Ok(links)
}

fn read_sentences(path: &Path, wanted: &HashSet<u32>) -> Result<HashMap<u32, String>, BoxError> {
    let mut out = HashMap::new();
    read_tsv(path, |c| {
        if let (Some(Ok(id)), Some(text)) = (c.first().map(|s| s.parse::<u32>()), c.get(2))
            && wanted.contains(&id)
        {
            out.insert(id, text.to_string());
        }
    })?;
    Ok(out)
}

fn read_tatoeba(dir: &Path, lex: &Lexicon) -> Result<Vec<Example>, BoxError> {
    let t = Instant::now();
    let mut langs = Vec::new();
    for (lang, code) in [("uk", "ukr"), ("en", "eng")] {
        let links = read_links(&dir.join(format!("deu-{code}_links.tsv")))?;
        let wanted: HashSet<u32> = links.values().copied().collect();
        let texts = read_sentences(&dir.join(format!("{code}_sentences.tsv")), &wanted)?;
        println!("tatoeba: {} deu-{code} links, {} {code} sentences", links.len(), texts.len());
        langs.push((lang, links, texts));
    }

    let mut out = Vec::new();
    let mut total = 0usize;
    read_tsv(&dir.join("deu_sentences_detailed.tsv"), |c| {
        let (Some(id), Some(text)) = (c.first(), c.get(2)) else { return };
        total += 1;
        let toks = tokenize(text);
        if toks.len() > MAX_WORDS {
            return;
        }
        let links = lex.match_verbs(&toks);
        if links.is_empty() {
            return;
        }
        let num: u32 = id.parse().unwrap_or(0);
        let translations = langs
            .iter()
            .filter_map(|(lang, links, texts)| {
                let tr = texts.get(links.get(&num)?)?;
                Some((*lang, tr.clone(), "tatoeba"))
            })
            .collect();
        out.push(Example {
            source_ref: id.to_string(),
            text: text.to_string(),
            author: c.get(3).filter(|a| **a != "\\N" && !a.is_empty()).map(|a| a.to_string()),
            citation: None,
            sense: None,
            word_count: toks.len(),
            rarity: lex.rarity(&toks),
            translations,
            links,
        });
    })?;
    println!("tatoeba: {} of {} sentences contain a known verb ({:?})", out.len(), total, t.elapsed());
    Ok(out)
}

/// Highlighted words from kaikki `bold_text_offsets` (offsets are in Unicode code points).
fn bold_words(text: &str, ex: &Value) -> Option<String> {
    let spans = ex.get("bold_text_offsets")?.as_array()?;
    let chars: Vec<char> = text.chars().collect();
    let words: Vec<String> = spans
        .iter()
        .filter_map(|s| {
            let (a, b) = (s.get(0)?.as_u64()? as usize, s.get(1)?.as_u64()? as usize);
            (a < b && b <= chars.len()).then(|| chars[a..b].iter().collect::<String>())
        })
        .flat_map(|w| w.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .collect();
    (!words.is_empty()).then(|| words.join(" "))
}

fn read_wiktionary(reader: impl BufRead, lex: &Lexicon, source: &str) -> Result<Vec<Example>, BoxError> {
    let t = Instant::now();
    let mut out: Vec<Example> = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let (mut entries, mut skipped_lemma) = (0usize, HashSet::new());

    for line in reader.lines() {
        let line = line?;
        // cheap pre-filter before parsing: the German dump contains every language and part of speech
        if !line.contains("\"lang_code\": \"de\"") && !line.contains("\"lang_code\":\"de\"") {
            continue;
        }
        let entry: Value = serde_json::from_str(&line)?;
        if entry["lang_code"] != "de" || entry["pos"] != "verb" {
            continue;
        }
        let Some(word) = entry["word"].as_str() else { continue };
        let Some(senses) = entry["senses"].as_array() else { continue };
        let has_examples = senses.iter().any(|s| s["examples"].as_array().is_some_and(|e| !e.is_empty()));
        if !has_examples {
            continue;
        }
        let Some(&verb_id) = lex.verbs.get(word) else {
            skipped_lemma.insert(word.to_string());
            continue;
        };
        entries += 1;

        for sense in senses {
            let gloss = sense["glosses"].as_array().and_then(|g| g.first()).and_then(Value::as_str);
            let Some(examples) = sense["examples"].as_array() else { continue };
            for ex in examples {
                let Some(text) = ex["text"].as_str().map(str::trim).filter(|t| !t.is_empty()) else { continue };
                let toks = tokenize(text);
                if toks.len() > MAX_WORDS || !seen.insert((word.to_string(), truncate(text, 191))) {
                    continue;
                }
                let form = bold_words(text, ex).or_else(|| {
                    lex.match_verbs(&toks).into_iter().find(|(id, _)| *id == verb_id).map(|(_, f)| f)
                });
                let translation = ex["english"]
                    .as_str()
                    .or_else(|| ex["translation"].as_str())
                    .map(str::trim)
                    .filter(|t| !t.is_empty() && source == "wiktionary_en");
                out.push(Example {
                    source_ref: word.to_string(),
                    text: text.to_string(),
                    author: None,
                    citation: ex["ref"].as_str().map(|r| r.trim().to_string()).filter(|r| !r.is_empty()),
                    sense: gloss.map(str::to_string),
                    word_count: toks.len(),
                    rarity: lex.rarity(&toks),
                    translations: translation.map(|tr| ("en", tr.to_string(), "wiktionary")).into_iter().collect(),
                    links: vec![(verb_id, form.unwrap_or_default())],
                });
            }
        }
    }
    println!(
        "{source}: {} examples from {} verb entries, {} lemmas with examples not in verben_wide ({:?})",
        out.len(),
        entries,
        skipped_lemma.len(),
        t.elapsed()
    );
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// Writing

async fn write_source(pool: &MySqlPool, source: &str, exs: Vec<Example>) -> Result<(), BoxError> {
    let t = Instant::now();
    // ON DELETE CASCADE removes the translations and verb links of this source too
    let deleted = sqlx::query("DELETE FROM example_sentences WHERE source = ?")
        .bind(source)
        .execute(pool)
        .await?
        .rows_affected();
    let next: i64 = sqlx::query_scalar("SELECT CAST(COALESCE(MAX(id), 0) + 1 AS SIGNED) FROM example_sentences")
        .fetch_one(pool)
        .await?;
    println!("{source}: deleted {deleted} old rows, writing {} examples", exs.len());

    let (mut n_tr, mut n_links) = (0usize, 0usize);
    for (chunk_no, chunk) in exs.chunks(BATCH).enumerate() {
        let base = next + (chunk_no * BATCH) as i64;
        let mut tx = pool.begin().await?;

        let mut q: QueryBuilder<MySql> = QueryBuilder::new(
            "INSERT INTO example_sentences (id, source, source_ref, text, author, citation, sense, word_count, rarity) ",
        );
        q.push_values(chunk.iter().enumerate(), |mut b, (i, e)| {
            b.push_bind(base + i as i64)
                .push_bind(source)
                .push_bind(&e.source_ref)
                .push_bind(&e.text)
                .push_bind(&e.author)
                .push_bind(&e.citation)
                .push_bind(&e.sense)
                .push_bind(e.word_count as i32)
                .push_bind(e.rarity as i32);
        });
        q.build().execute(&mut *tx).await?;

        let trs: Vec<(i64, &(&str, String, &str))> = chunk
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.translations.iter().map(move |t| (base + i as i64, t)))
            .collect();
        if !trs.is_empty() {
            let mut q: QueryBuilder<MySql> =
                QueryBuilder::new("INSERT INTO example_translations (example_id, lang, text, origin) ");
            q.push_values(&trs, |mut b, (id, (lang, text, origin))| {
                b.push_bind(id).push_bind(*lang).push_bind(text).push_bind(*origin);
            });
            q.build().execute(&mut *tx).await?;
            n_tr += trs.len();
        }

        let links: Vec<(i64, &(i32, String))> = chunk
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.links.iter().map(move |l| (base + i as i64, l)))
            .collect();
        for part in links.chunks(BATCH * 4) {
            let mut q: QueryBuilder<MySql> = QueryBuilder::new("INSERT INTO verb_examples (verb_id, example_id, form) ");
            q.push_values(part, |mut b, (id, (verb_id, form))| {
                b.push_bind(*verb_id).push_bind(id).push_bind(truncate(form, 200));
            });
            q.build().execute(&mut *tx).await?;
            n_links += part.len();
        }

        tx.commit().await?;
        if chunk_no % 50 == 49 {
            println!("{source}: {} / {}", (chunk_no + 1) * BATCH, exs.len());
        }
    }
    println!(
        "{source}: wrote {} examples, {n_tr} translations, {n_links} verb links ({:?})",
        exs.len(),
        t.elapsed()
    );
    Ok(())
}

fn truncate(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}
