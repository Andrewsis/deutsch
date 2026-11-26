<script lang="ts">
    import { apiUrl } from "$lib/api";
    import { t } from "$lib/i18n";

    type Sentence = {
        text: string;
        form: string;
        translation: string | null;
        source: string;
        source_url: string;
        author: string | null;
        citation: string | null;
        sense: string | null;
    };

    type Part = { text: string; hit: boolean };

    /** Verb lemma. */
    export let verb: string;
    /** First page, loaded by +page.ts. */
    export let initial: Sentence[] = [];
    export let lang: string;

    const PAGE = 10;

    let items: Sentence[] = [];
    let hasMore = false;
    let loading = false;

    $: reset(initial);

    function reset(first: Sentence[]) {
        items = first;
        hasMore = first.length === PAGE;
    }

    async function loadMore() {
        loading = true;
        try {
            const res = await fetch(
                apiUrl(
                    `/api/sentences?verb=${encodeURIComponent(verb)}&lang=${lang}&limit=${PAGE}&offset=${items.length}`,
                ),
            );
            if (res.ok) {
                const next: Sentence[] = await res.json();
                items = [...items, ...next];
                hasMore = next.length === PAGE;
            }
        } finally {
            loading = false;
        }
    }

    /** Splits the sentence into parts, marking whole words that belong to `form`. */
    function highlight(text: string, form: string): Part[] {
        const words = new Set(form.split(/\s+/).filter(Boolean));
        if (words.size === 0) return [{ text, hit: false }];
        return text
            .split(/([\p{L}]+)/u)
            .filter((p) => p !== "")
            .map((p) => ({ text: p, hit: words.has(p) }));
    }

    const SOURCE_NAMES: Record<string, string> = {
        tatoeba: "Tatoeba",
        wiktionary_en: "Wiktionary",
        wiktionary_de: "Wiktionary (de)",
    };
</script>

{#if items.length > 0}
    <section
        class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-3 md:p-6 shadow-sm"
    >
        <h4
            class="text-[11px] md:text-sm font-bold text-stone-400 dark:text-stone-500 uppercase tracking-wider mb-3 md:mb-4"
        >
            {$t("word.sentences")}
        </h4>

        <ul class="divide-y divide-stone-100 dark:divide-stone-800">
            {#each items as s}
                <li class="py-2.5 md:py-3 first:pt-0 last:pb-0">
                    <p class="text-[15px] md:text-base text-stone-900 dark:text-stone-100 leading-snug">
                        {#each highlight(s.text, s.form) as part}{#if part.hit}<mark
                                    class="bg-orange-100 dark:bg-orange-900/40 text-orange-800 dark:text-orange-200 font-semibold rounded px-0.5"
                                    >{part.text}</mark
                                >{:else}{part.text}{/if}{/each}
                    </p>
                    {#if s.translation}
                        <p class="text-[13px] md:text-sm text-stone-500 dark:text-stone-400 mt-0.5 leading-snug">
                            {s.translation}
                        </p>
                    {/if}
                    <p class="text-[11px] text-stone-400 dark:text-stone-500 mt-1 flex flex-wrap gap-x-2">
                        {#if s.sense}<span class="italic">{s.sense}</span>{/if}
                        {#if s.citation}<span class="truncate max-w-full">{s.citation}</span>{/if}
                        <a
                            href={s.source_url}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="hover:text-orange-500 underline underline-offset-2 decoration-stone-300 dark:decoration-stone-700"
                            title={$t("word.source")}
                            >{SOURCE_NAMES[s.source] ?? s.source}{#if s.author}&nbsp;· {s.author}{/if}</a
                        >
                    </p>
                </li>
            {/each}
        </ul>

        {#if hasMore}
            <button
                on:click={loadMore}
                disabled={loading}
                class="mt-3 md:mt-4 text-sm font-medium text-orange-500 hover:text-orange-600 disabled:opacity-50 transition-colors"
            >
                {loading ? $t("word.loading") : $t("word.moreSentences")}
            </button>
        {/if}
    </section>
{/if}
