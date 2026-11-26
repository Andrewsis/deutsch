# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

A German-verb learning app. German is the language being learned; the UI and verb translations are localized (currently Ukrainian `uk` and English `en`). Three independent projects share one MySQL database; there is no workspace or shared crate between them.

- `backend/` — Rust (edition 2024) Axum API server on `0.0.0.0:8080`.
- `my-skeleton-app/` — SvelteKit (Svelte 4) + Skeleton UI + Tailwind frontend on Vite's port 5173.
- `tools/translate/` — Node/TypeScript script that generates verb translations for a new language with the Claude API (see below).
- `scraper/` — standalone Rust binary that pulls German news RSS feeds (Spiegel, Zeit, Tagesschau, SZ), counts word/bigram frequencies and writes them to `word_stats`.

There are no tests and no lint config beyond `cargo clippy` / `svelte-check`.

## Commands

```sh
# backend (needs .env with DATABASE_URL and JWT_SECRET)
cd backend && cargo run --bin backend         # API server
cd backend && cargo run --bin <name>         # one-off data loaders, see below
cd backend && cargo check / cargo clippy

# scraper (needs DATABASE_URL)
cd scraper && cargo run

# frontend
cd my-skeleton-app && npm run dev            # add `-- --host` to reach it from the LAN
cd my-skeleton-app && npm run check          # svelte-check type checking
cd my-skeleton-app && npm run build
```

**Docker:** `cp .env.example .env` (fill in secrets), then `docker compose up -d --build` serves the app on `http://localhost` (`nginx` service, config `nginx/default.conf`, proxies `/api` incl. WebSocket to `backend` and everything else to `frontend`, the SvelteKit Node SSR server on port 3000). Set `PUBLIC_ORIGIN` in `.env` to the public site URL: it goes into canonical/hreflang links and the sitemap. MariaDB 11.4 is initialised from `db/schema.sql` + `db/seed.sql` (seed is gitignored, must exist locally) only when the `db_data` volume is empty; `docker compose down -v` wipes it for a re-init.

**sqlx compile-time checking:** the Rust code uses `sqlx::query!` / `query_as!` macros. The query metadata is cached in `backend/.sqlx/` and `scraper/.sqlx/`, so `SQLX_OFFLINE=true cargo build` works without a database (this is how Docker builds). Without `SQLX_OFFLINE`, builds connect to `DATABASE_URL`. After changing a macro query or the schema, apply the schema to the DB, then run `cargo sqlx prepare -- --all-targets` in `backend/` (or `cargo sqlx prepare` in `scraper/`) and commit `.sqlx/`; `cargo sqlx prepare --check` verifies the cache is current.

## Backend architecture

- `main.rs` wires routes, all under `/api`. Allowed CORS origins (with credentials) come from `CORS_ORIGINS` (comma-separated, default `http://localhost:5173`); listen address from `BIND_ADDR` (default `0.0.0.0:8080`). See `backend/.env.example`.
- `state.rs`: `AppState` = MySQL pool + `waiting_player`, a single-slot matchmaking lobby.
- `auth.rs`: Argon2 password hashing; login/register set an HttpOnly `jwt` cookie (24h, HS256 with `JWT_SECRET`). `Claims` implements `FromRequestParts`, so adding a `claims: Claims` parameter to any handler makes it require authentication (username is `claims.sub`).
- `lang.rs`: `LangQuery` parses the `?lang=` query parameter (whitelist `SUPPORTED`, unknown/missing → `uk`). Word, flashcard and arena endpoints take it and read `verb_translations`, falling back to `uk` when the requested language has no row.
- `words.rs`: `GET /api/word/{word}?lang=` returns the full conjugation row from `verben_wide` plus `translation` and `prepositions` (`[{phrase, text}]`); `GET /api/lemmas` lists all verb lemmas (for the sitemap); `GET /api/examples?word=&limit=&offset=` does a MySQL FULLTEXT boolean phrase search on `subtitles` and returns YouTube `video_id` + `start_sec` (the frontend embeds YouTube at that timestamp).
- `flashcards.rs`: SM-2 spaced repetition (`calculate_sm2`), state stored in `user_flashcards` (upsert via `ON DUPLICATE KEY UPDATE`). Note that `get_due_flashcards` currently returns 10 random verbs and does not yet use `next_review`.
- `ws/`: 1v1 "arena" quiz over WebSocket at `/api/ws/match` (auth via cookie). `ws/mod.rs` pairs players: the first connection parks a `oneshot::Sender` in the lobby and awaits; the second sends its socket through it, and the first player's task spawns `game::run_game`, which owns both sockets. `/api/ws/match?lang=` — each player can use a different language. `game.rs` builds 5 questions (lemma → 4 translation options, per player in their language) and exchanges JSON messages tagged by `action`: `start`, `score_update`, `finish` (client sends `{index, answer}`).
- `stats.rs`: `GET /api/stats/words` returns the top words/bigrams from the most recent `scraped_at` run of the scraper.

### Data-loading binaries (`backend/src/bin/`)

One-off scripts used to build the database (not part of the server); each reads a file from the current working directory. They use runtime `sqlx::query()` (not in `.sqlx/`), and `peek`/`parser` need the `verb_meta` / `verb_forms_kaikki` tables, which exist only in the old `deutsch_mdb`, not in `db/schema.sql`:

- `peek.rs` / `parser.rs` — import verbs/forms from kaikki.org Wiktionary JSONL dumps (`kaikki.org-dictionary-German.jsonl`, `german_verbs.jsonl`).
- `timecode_db_load.rs` — loads `database_ready.json` (YouTube videos + subtitle timecodes) into `videos` / `subtitles`.
- `sync_audio.rs` — for every verb in `verben_wide`, finds which forms appear in `subtitles` and stores them in `audio_forms` (JSON).

### Key tables

`verben_wide` (one wide row per verb: `lemma`, `aux`, columns per tense/person such as `prs1sg`, `prt3pl`, `kj1_*`, `kj2_*`, `fut1_*`, `fut2_*`, `imp2sg`, `pa1`, `pa2`, plus `audio_forms` JSON; legacy `translation_ua` / `praepositionen_ua` are superseded), `verb_translations` (`verb_id`, `lang`, `translation`, `prepositions` JSON `[{phrase, text}]`, PK `(verb_id, lang)`), `users`, `user_flashcards`, `videos`, `subtitles` (FULLTEXT index on `text`), `word_stats` (created by the scraper's own migration).

### Translation generator (`tools/translate/`)

`npx tsx translate.ts --lang en [--limit 30]` reads verbs + Ukrainian hints from `verb_translations` (SELECT only), asks Claude Opus 5.5 (structured outputs, batches of 40) and accumulates results in `out/verbs_<lang>.json` (re-runs skip finished verbs, failures go to `out/failures_<lang>.json`). It writes `out/verb_translations_<lang>.sql`, which is applied to the DB separately. Needs `ANTHROPIC_API_KEY` in `tools/translate/.env`; `DATABASE_URL` defaults to `backend/.env`. Costs real money: run a `--limit` trial first.

## Frontend architecture

- Routes: `/` (home, verb search, word stats), `/[word]` (verb page: conjugation tables + subtitle video examples, loaded in `+page.ts`), `/study` (flashcards), `/arena` (WebSocket game), `/login`, `/register`, plus `sitemap.xml` and `robots.txt` (`+server.ts`).
- SSR / SEO: `@sveltejs/adapter-node`. `/` and `/[word]` render on the server (meta tags via `Seo.svelte`, texts from `src/lib/seo.ts` and `seo.*` keys); `/study`, `/arena`, `/login`, `/register` set `ssr = false`. Code in components and universal loads must not touch `window`/`localStorage` outside `onMount`. `hooks.server.ts` picks the language and, when `API_INTERNAL_URL` is set (Docker), sends SSR `/api` fetches straight to the backend. Redirects from inflected forms are 301.
- i18n: `src/lib/i18n/` — language per request: `?lang=` > `locale` cookie > `Accept-Language` > `uk` (`resolveLocale`, in `hooks.server.ts`); the root layout server load returns it as `data.lang` and `+layout.svelte` puts it into the `locale` store, which writes the cookie back in the browser. The default language has URLs without `?lang=`, other languages `?lang=xx` (canonical + hreflang). The store is shared between SSR requests, so load functions take `lang` from `await parent()` and use `withLang(path, lang)` / `translate(lang, …)`, never `get(locale)` / `tr()`. `$t("key", params)` in components. Dictionaries `uk.ts` (source of key names) and `en.ts` (typed `Record<MessageKey, string>`, so a missing key fails `npm run check`). `LangSwitcher.svelte` changes the locale and calls `invalidate("app:lang")`; loaders that fetch translations call `depends("app:lang")`. Adding a language: dictionary + `LOCALES` entry here, `SUPPORTED` in `backend/src/lang.rs`, `LANG_NAMES` in `tools/translate/translate.ts`.
- API URLs are built with `apiUrl()` / `wsUrl()` from `src/lib/api.ts`, `withLang(path)` appends the current `lang`; based on `PUBLIC_API_URL` (`my-skeleton-app/.env`, see `.env.example`). Empty `PUBLIC_API_URL` means same origin (`/api/...`, prod behind nginx). Requests use `credentials: "include"` for the cookie; the frontend origin must be listed in the backend's `CORS_ORIGINS`.

## Git

- Перед началом задачи проверь `git status`. Если есть несохранённые изменения, скажи мне.
- Работай в отдельной ветке `feature/...` или `fix/...`, в `main` не коммить.
- Коммить после каждого законченного шага. Сообщения на английском: `feat:`, `fix:`, `refactor:`, `chore:`.
- Перед коммитом запускай `cargo clippy` в `backend/` и `scraper/`, а если менялся фронтенд, то `npm run check` в `my-skeleton-app/`.
- Не коммить `.env`, `target/`, `node_modules/` и большие файлы данных (`*.jsonl`, `database_ready.json`).
- Не делай `git push`, `git reset --hard` и `push --force` без моего разрешения.

## Осторожно с данными

- Не запускай бинарники из `backend/src/bin/` и `scraper` без моего прямого разрешения: они пишут в базу.
- SQL (в том числе изменения схемы) можно выполнять самому, не спрашивая. Изменения схемы сохраняй файлом в `db/migrations/` и в `db/schema.sql`.
- После изменений запросов `sqlx::query!` запусти `cargo sqlx prepare` и закоммить `.sqlx/`.
