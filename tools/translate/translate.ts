/**
 * Generates verb translations for a new language with Claude, using the Ukrainian
 * data in `verb_translations` as a meaning hint.
 *
 *   npx tsx translate.ts --lang en --limit 30   # trial run on the first 30 verbs
 *   npx tsx translate.ts --lang en              # everything still missing
 *   npx tsx translate.ts --lang en --sql-only   # only rebuild the SQL file
 *
 * Reads the DB (SELECT only). Results accumulate in out/verbs_<lang>.json (re-runs skip
 * verbs that are already there) and are written to out/verb_translations_<lang>.sql,
 * which is applied to the database separately.
 */
import { config as loadEnv } from "dotenv";
import fs from "node:fs";
import path from "node:path";
import { parseArgs } from "node:util";
import Anthropic from "@anthropic-ai/sdk";
import { betaZodOutputFormat } from "@anthropic-ai/sdk/helpers/beta/zod";
import mysql from "mysql2/promise";
import { z } from "zod";

loadEnv({ path: path.resolve(import.meta.dirname, ".env") });
loadEnv({ path: path.resolve(import.meta.dirname, "../../backend/.env") }); // DATABASE_URL fallback

const LANG_NAMES: Record<string, string> = { en: "English" };
const MODEL = "claude-opus-5-5";
const PRICE_IN = 4 / 1e6; // $ per input token, Claude Opus 5.5
const PRICE_OUT = 20 / 1e6; // $ per output token

const { values: args } = parseArgs({
  options: {
    lang: { type: "string", default: "en" },
    limit: { type: "string" },
    batch: { type: "string", default: "40" },
    concurrency: { type: "string", default: "4" },
    "sql-only": { type: "boolean", default: false },
  },
});

const lang = args.lang!;
const langName = LANG_NAMES[lang];
if (!langName) throw new Error(`Unknown --lang ${lang}; add it to LANG_NAMES`);

const outDir = path.resolve(import.meta.dirname, "out");
const jsonPath = path.join(outDir, `verbs_${lang}.json`);
const sqlPath = path.join(outDir, `verb_translations_${lang}.sql`);
fs.mkdirSync(outDir, { recursive: true });

type Prep = { phrase: string; text: string };
type Result = { translation: string; prepositions: Prep[] };
const results: Record<string, Result> = fs.existsSync(jsonPath)
  ? JSON.parse(fs.readFileSync(jsonPath, "utf8"))
  : {};

type Verb = { id: number; lemma: string; uk: string | null; preps: Prep[] };

const ResponseSchema = z.object({
  items: z.array(
    z.object({
      id: z.number().int(),
      translation: z.string(),
      prepositions: z.array(z.object({ phrase: z.string(), text: z.string() })),
    }),
  ),
});

const SYSTEM = `You are a lexicographer writing a German verb dictionary for learners whose interface language is ${langName}.
For each German verb you get its lemma, a Ukrainian translation (a hint for which meanings to cover; it may be missing) and a list of German prepositional constructions ("Rektion").

Return for every verb:
- "translation": a concise ${langName} dictionary gloss covering the main meanings, most common first, separated by ", " or "; " for clearly different senses. ${lang === "en" ? 'Use the "to ..." infinitive form, lowercase, e.g. "to make, to do" or "to wait; to service (machinery)".' : ""} No German words, no explanations, no quotes.
- "prepositions": one entry per input construction, in the same order. Copy "phrase" exactly as given (same characters). "text" is the ${langName} equivalent in the same style as the Ukrainian hint, keeping placeholders for the object, e.g. "denken an (Akk)" -> "to think of/about (sb/sth)".

Return every input verb exactly once with the same "id". If a verb has no constructions, return an empty "prepositions" array.`;

async function loadVerbs(): Promise<Verb[]> {
  const url = process.env.DATABASE_URL;
  if (!url) throw new Error("DATABASE_URL is not set");
  const conn = await mysql.createConnection(url);
  try {
    const [rows] = await conn.query(
      `SELECT v.id, v.lemma, t.translation, t.prepositions
       FROM verben_wide v
       LEFT JOIN verb_translations t ON t.verb_id = v.id AND t.lang = 'uk'
       ORDER BY v.id`,
    );
    return (rows as any[]).map((r) => {
      const raw = r.prepositions;
      const preps: Prep[] = raw ? (typeof raw === "string" ? JSON.parse(raw) : raw) : [];
      return { id: r.id, lemma: r.lemma, uk: r.translation, preps };
    });
  } finally {
    await conn.end();
  }
}

function validate(batch: Verb[], items: z.infer<typeof ResponseSchema>["items"]) {
  const byId = new Map(items.map((i) => [i.id, i]));
  const ok: [Verb, Result][] = [];
  const failed: { verb: Verb; reason: string }[] = [];
  for (const verb of batch) {
    const item = byId.get(verb.id);
    if (!item) {
      failed.push({ verb, reason: "missing in response" });
      continue;
    }
    const translation = item.translation.trim();
    if (!translation) {
      failed.push({ verb, reason: "empty translation" });
      continue;
    }
    const phrasesMatch =
      item.prepositions.length === verb.preps.length &&
      item.prepositions.every((p, i) => p.phrase === verb.preps[i].phrase && p.text.trim() !== "");
    if (!phrasesMatch) {
      failed.push({ verb, reason: "prepositions do not match input" });
      continue;
    }
    ok.push([verb, { translation, prepositions: item.prepositions.map((p) => ({ phrase: p.phrase, text: p.text.trim() })) }]);
  }
  return { ok, failed };
}

const client = new Anthropic({ maxRetries: 5 });
let inputTokens = 0;
let outputTokens = 0;

async function translateBatch(batch: Verb[]) {
  const payload = batch.map((v) => ({
    id: v.id,
    lemma: v.lemma,
    ukrainian: v.uk,
    constructions: v.preps.map((p) => ({ phrase: p.phrase, ukrainian: p.text })),
  }));

  const response = await client.beta.messages.parse({
    model: MODEL,
    max_tokens: 16000,
    betas: ["server-side-fallback-2026-07-01"],
    fallbacks: "default",
    output_config: { effort: "medium", format: betaZodOutputFormat(ResponseSchema) },
    system: SYSTEM,
    messages: [{ role: "user", content: JSON.stringify(payload) }],
  });

  inputTokens += response.usage.input_tokens;
  outputTokens += response.usage.output_tokens;

  if (response.stop_reason === "refusal") {
    throw new Error(`refusal: ${response.stop_details?.category ?? "unknown"}`);
  }
  if (response.stop_reason === "max_tokens") throw new Error("hit max_tokens");
  if (!response.parsed_output) throw new Error("could not parse response");
  return validate(batch, response.parsed_output.items);
}

function save() {
  fs.writeFileSync(jsonPath, JSON.stringify(results, null, 1));
}

function sqlString(s: string) {
  return "'" + s.replace(/\\/g, "\\\\").replace(/'/g, "''") + "'";
}

function writeSql() {
  const ids = Object.keys(results).map(Number).sort((a, b) => a - b);
  const lines = ids.map((id) => {
    const r = results[id];
    return `(${id},'${lang}',${sqlString(r.translation)},${sqlString(JSON.stringify(r.prepositions))})`;
  });
  const chunks: string[] = [];
  for (let i = 0; i < lines.length; i += 500) {
    chunks.push(
      "INSERT INTO verb_translations (verb_id, lang, translation, prepositions) VALUES\n" +
        lines.slice(i, i + 500).join(",\n") +
        "\nON DUPLICATE KEY UPDATE translation = VALUES(translation), prepositions = VALUES(prepositions);",
    );
  }
  fs.writeFileSync(sqlPath, "SET NAMES utf8mb4;\n" + chunks.join("\n") + "\n");
  console.log(`SQL: ${sqlPath} (${ids.length} verbs)`);
}

async function main() {
  if (args["sql-only"]) return writeSql();

  const all = await loadVerbs();
  let todo = all.filter((v) => !results[v.id]);
  if (args.limit) todo = todo.slice(0, Number(args.limit));
  console.log(`${all.length} verbs, ${Object.keys(results).length} done, translating ${todo.length} -> ${langName}`);

  const batchSize = Number(args.batch);
  const batches: Verb[][] = [];
  for (let i = 0; i < todo.length; i += batchSize) batches.push(todo.slice(i, i + batchSize));

  const failures: { verb: Verb; reason: string }[] = [];
  let next = 0;
  let done = 0;

  async function worker() {
    while (next < batches.length) {
      const batch = batches[next++];
      try {
        const { ok, failed } = await translateBatch(batch);
        for (const [verb, result] of ok) results[verb.id] = result;
        failures.push(...failed);
        save();
      } catch (e) {
        if (e instanceof Anthropic.AuthenticationError) throw e;
        const reason = e instanceof Error ? e.message : String(e);
        console.error(`batch ${batch[0].id}..${batch.at(-1)!.id} failed: ${reason}`);
        failures.push(...batch.map((verb) => ({ verb, reason })));
      }
      done++;
      const cost = inputTokens * PRICE_IN + outputTokens * PRICE_OUT;
      console.log(`[${done}/${batches.length}] tokens in ${inputTokens}, out ${outputTokens}, ~$${cost.toFixed(2)}`);
    }
  }

  await Promise.all(Array.from({ length: Number(args.concurrency) }, worker));

  if (failures.length) {
    const failPath = path.join(outDir, `failures_${lang}.json`);
    fs.writeFileSync(failPath, JSON.stringify(failures.map((f) => ({ id: f.verb.id, lemma: f.verb.lemma, reason: f.reason })), null, 1));
    console.log(`${failures.length} verbs failed, see ${failPath}; re-run to retry them`);
  }
  writeSql();
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
