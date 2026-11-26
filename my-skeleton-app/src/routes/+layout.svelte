<script lang="ts">
    import "../app.postcss";
    import { AppShell } from "@skeletonlabs/skeleton";
    import { page } from "$app/stores";
    import LangSwitcher from "$lib/components/LangSwitcher.svelte";

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
