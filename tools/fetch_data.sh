#!/usr/bin/env sh
# Downloads raw example-sentence sources into data/raw/ (gitignored).
# Re-running skips files that already exist.
set -eu
cd "$(dirname "$0")/.."
mkdir -p data/raw
cd data/raw

fetch() { # url [file]
  f=${2:-$(basename "$1")}
  if [ -e "$f" ] || [ -e "${f%.bz2}" ]; then echo "skip $f"; return; fi
  echo "get  $f"
  curl -fL --retry 3 -o "$f.part" "$1" && mv "$f.part" "$f"
}

T=https://downloads.tatoeba.org/exports/per_language
fetch $T/deu/deu_sentences_detailed.tsv.bz2
fetch $T/deu/deu-ukr_links.tsv.bz2
fetch $T/deu/deu-eng_links.tsv.bz2
fetch $T/ukr/ukr_sentences.tsv.bz2
fetch $T/eng/eng_sentences.tsv.bz2

# English Wiktionary, German verbs (examples with English translations)
fetch https://kaikki.org/dictionary/German/pos-verb/kaikki.org-dictionary-German-by-pos-verb.jsonl
# German Wiktionary, all entries (read as a gz stream, not unpacked)
fetch https://kaikki.org/dewiktionary/raw-wiktextract-data.jsonl.gz dewiktionary.jsonl.gz

for f in *.bz2; do [ -e "$f" ] && bzip2 -d "$f"; done
ls -la
