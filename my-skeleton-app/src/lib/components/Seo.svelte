<script lang="ts">
    import { page } from "$app/stores";
    import { DEFAULT_LOCALE, LOCALES, locale, type Locale } from "$lib/i18n";

    export let title: string;
    export let description: string;
    /** Site path without query, e.g. `/gehen`. Defaults to the current path. */
    export let path: string | undefined = undefined;

    const OG_LOCALES: Record<Locale, string> = { uk: "uk_UA", en: "en_US" };

    // One URL per language: the default language has no `?lang=`, the others do.
    // The origin comes from adapter-node (ORIGIN env in production).
    function urlFor(base: string, lang: Locale): string {
        return lang === DEFAULT_LOCALE ? base : `${base}?lang=${lang}`;
    }

    $: base = `${$page.url.origin}${path ?? $page.url.pathname}`;
    $: canonical = urlFor(base, $locale);
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />
    <link rel="canonical" href={canonical} />
    {#each LOCALES as l}
        <link rel="alternate" hreflang={l.code} href={urlFor(base, l.code)} />
    {/each}
    <link rel="alternate" hreflang="x-default" href={urlFor(base, DEFAULT_LOCALE)} />
    <meta property="og:type" content="website" />
    <meta property="og:site_name" content="DeutschDB" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:url" content={canonical} />
    <meta property="og:locale" content={OG_LOCALES[$locale]} />
</svelte:head>
