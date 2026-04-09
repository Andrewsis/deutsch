<script lang="ts">
    import { onMount } from "svelte";

    type WordStat = {
        word: string;
        is_bigram: boolean;
        count: number;
    };

    let words: WordStat[] = [];
    let bigrams: WordStat[] = [];
    let scrapedAt = "";
    let loading = true;
    let error = "";

    onMount(async () => {
        try {
            const res = await fetch(
                `http://${window.location.hostname}:8080/api/stats/words`,
                { credentials: "include" },
            );
            if (!res.ok) throw new Error("Failed to fetch");
            const data = await res.json();
            words = data.words ?? [];
            bigrams = data.bigrams ?? [];
            scrapedAt = data.scraped_at ?? "";
        } catch (e) {
            error = "Не вдалося завантажити статистику";
        } finally {
            loading = false;
        }
    });

    function fontSize(count: number, max: number): string {
        const min = 0.8;
        const maxSize = 2.4;
        const size = min + (count / max) * (maxSize - min);
        return `${size.toFixed(2)}rem`;
    }

    function color(count: number, max: number): string {
        const ratio = count / max;
        if (ratio > 0.7) return "text-orange-500 dark:text-orange-400";
        if (ratio > 0.4) return "text-orange-400 dark:text-orange-500";
        if (ratio > 0.2) return "text-stone-600 dark:text-stone-300";
        return "text-stone-400 dark:text-stone-500";
    }

    $: maxCount = words.length > 0 ? words[0].count : 1;
    $: maxBigramCount = bigrams.length > 0 ? bigrams[0].count : 1;

    function formatDate(iso: string): string {
        if (!iso) return "";
        return new Date(iso).toLocaleString("de-DE", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric",
            hour: "2-digit",
            minute: "2-digit",
        });
    }
</script>

<section class="max-w-3xl mx-auto px-4 py-12 space-y-10">
    <div class="text-center space-y-1">
        <h2 class="text-2xl font-extrabold text-stone-900 dark:text-white">
            Deutsche Nachrichten — Trending
        </h2>
        {#if scrapedAt}
            <p class="text-xs text-stone-400">
                Оновлено: {formatDate(scrapedAt)}
            </p>
        {/if}
    </div>

    {#if loading}
        <div class="flex justify-center py-12">
            <div
                class="w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"
            ></div>
        </div>
    {:else if error}
        <p class="text-center text-stone-400">{error}</p>
    {:else}
        <!-- Облако слов -->
        <div>
            <h3
                class="text-xs uppercase tracking-widest text-stone-400 mb-4 font-semibold"
            >
                Популярні слова
            </h3>
            <div class="flex flex-wrap gap-x-4 gap-y-2 leading-relaxed">
                {#each words as word}
                    <span
                        class="font-semibold transition-opacity hover:opacity-80 cursor-default {color(
                            word.count,
                            maxCount,
                        )}"
                        style="font-size: {fontSize(word.count, maxCount)}"
                        title="{word.count} разів"
                    >
                        {word.word}
                    </span>
                {/each}
            </div>
        </div>

        <div>
            <h3
                class="text-xs uppercase tracking-widest text-stone-400 mb-4 font-semibold"
            >
                Популярні словосполучення
            </h3>
            <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                {#each bigrams as bigram}
                    {@const ratio = bigram.count / maxBigramCount}
                    <div
                        class="flex items-center justify-between px-3 py-2 rounded-lg bg-stone-100 dark:bg-stone-900 border border-stone-200 dark:border-stone-800"
                    >
                        <span
                            class="text-sm font-medium text-stone-800 dark:text-stone-200 truncate"
                        >
                            {bigram.word}
                        </span>
                        <span
                            class="ml-2 text-xs font-bold shrink-0 {ratio > 0.5
                                ? 'text-orange-500'
                                : 'text-stone-400'}"
                        >
                            {bigram.count}
                        </span>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</section>
