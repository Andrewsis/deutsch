<script lang="ts">
    import "../app.postcss";
    import { AppShell } from "@skeletonlabs/skeleton";
    import { page } from "$app/stores";
    import LangSwitcher from "$lib/components/LangSwitcher.svelte";
    import { locale } from "$lib/i18n";
    import type { LayoutData } from "./$types";

    export let data: LayoutData;

    // The language the server chose for this request. During SSR this runs synchronously
    // before the page renders, so the shared store holds this request's language.
    $: locale.set(data.lang);

    // These pages have their own header with the language switcher.
    const ROUTES_WITH_HEADER = ["/", "/[word]"];
    $: showFloatingSwitcher = !ROUTES_WITH_HEADER.includes($page.route.id ?? "");
</script>

<AppShell>
    {#if showFloatingSwitcher}
        <div class="fixed top-3 right-3 z-20">
            <LangSwitcher />
        </div>
    {/if}
    <slot />
</AppShell>
