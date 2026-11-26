<script lang="ts">
    import { goto, invalidate } from "$app/navigation";
    import { page } from "$app/stores";
    import { LOCALES, locale, t, type Locale } from "$lib/i18n";

    function select(code: Locale) {
        if ($locale === code) return;
        // Also stores the cookie, so the server renders the next pages in this language.
        locale.set(code);
        if ($page.url.searchParams.has("lang")) {
            // `?lang=` beats the cookie on the server: rewrite it.
            const url = new URL($page.url);
            url.searchParams.set("lang", code);
            goto(url, { replaceState: true, noScroll: true, keepFocus: true, invalidate: ["app:lang"] });
        } else {
            invalidate("app:lang");
        }
    }
</script>

<div
    class="flex items-center rounded-lg bg-stone-100 dark:bg-stone-900 p-0.5 text-xs font-bold"
    role="group"
    aria-label={$t("common.language")}
>
    {#each LOCALES as l}
        <button
            type="button"
            on:click={() => select(l.code)}
            aria-pressed={$locale === l.code}
            class="px-2 py-1 rounded-md transition-colors {$locale === l.code
                ? 'bg-white dark:bg-stone-700 text-orange-600 dark:text-orange-400 shadow-sm'
                : 'text-stone-500 hover:text-stone-800 dark:hover:text-stone-200'}"
        >
            {l.label}
        </button>
    {/each}
</div>
