<script lang="ts">
    import WordStats from "$lib/components/WordStats.svelte";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import LangSwitcher from "$lib/components/LangSwitcher.svelte";
    import { t } from "$lib/i18n";

    let searchTerm = "";

    let isDark = false;

    onMount(() => {
        isDark = document.documentElement.classList.contains("dark");
    });

    function toggleTheme() {
        isDark = !isDark;
        if (isDark) {
            document.documentElement.classList.add("dark");
            localStorage.theme = "dark";
        } else {
            document.documentElement.classList.remove("dark");
            localStorage.theme = "light";
        }
    }

    function handleSearch() {
        if (searchTerm.trim()) {
            goto(`/${encodeURIComponent(searchTerm.trim())}`);
            searchTerm = "";
        }
    }
</script>

<div
    class="sticky top-0 z-10 bg-white/90 dark:bg-[#111]/90 backdrop-blur-md border-b border-stone-200 dark:border-stone-800"
>
    <div
        class="container mx-auto px-4 h-16 flex items-center justify-between gap-4"
    >
        <a
            href="/"
            class="text-xl font-extrabold tracking-tight text-stone-900 dark:text-white hover:opacity-70 transition-opacity"
        >
            Deutsch<span class="text-orange-500 font-light">DB</span>
        </a>

        <div class="flex-1 max-w-md">
            <form on:submit|preventDefault={handleSearch} class="relative">
                <input
                    type="search"
                    bind:value={searchTerm}
                    placeholder={$t("common.search")}
                    class="w-full bg-stone-100 dark:bg-stone-900 border border-transparent focus:border-orange-500 focus:bg-white dark:focus:bg-stone-950 focus:ring-0 rounded-lg pl-4 pr-10 py-2 text-sm transition-all outline-none text-stone-900 dark:text-stone-100 placeholder-stone-500"
                />
                <button
                    type="submit"
                    class="absolute right-3 top-1/2 -translate-y-1/2 text-stone-400 hover:text-orange-500 transition-colors"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="18"
                        height="18"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        ><circle cx="11" cy="11" r="8"></circle><line
                            x1="21"
                            y1="21"
                            x2="16.65"
                            y2="16.65"
                        ></line></svg
                    >
                </button>
            </form>
        </div>

        <div class="flex items-center gap-4">
            <div
                class="hidden sm:flex items-center gap-3 mr-2 border-r border-stone-200 dark:border-stone-800 pr-4"
            >
                <a
                    href="/study"
                    class="text-sm font-medium text-stone-600 dark:text-stone-300 hover:text-orange-500 transition-colors flex items-center gap-1.5"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"
                        ></rect>
                        <line x1="3" y1="9" x2="21" y2="9"></line>
                        <line x1="9" y1="21" x2="9" y2="9"></line>
                    </svg>
                    {$t("nav.flashcards")}
                </a>

                <a
                    href="/arena"
                    class="text-sm font-medium text-stone-600 dark:text-stone-300 hover:text-orange-500 transition-colors flex items-center gap-1.5"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"
                        ></polygon>
                    </svg>
                    {$t("nav.arena")}
                </a>

                <a
                    href="/login"
                    class="text-sm font-medium text-stone-600 dark:text-stone-300 hover:text-orange-500 transition-colors"
                >
                    {$t("common.login")}
                </a>
                <a
                    href="/register"
                    class="text-sm font-bold bg-orange-500 hover:bg-orange-600 text-white px-4 py-2 rounded-lg transition-colors shadow-sm"
                >
                    {$t("common.register")}
                </a>
            </div>
            <LangSwitcher />
            <button
                on:click={toggleTheme}
                aria-label={$t("common.toggleTheme")}
                class="text-stone-500 dark:text-stone-400 hover:text-orange-500 transition-colors p-2 rounded-full hover:bg-stone-100 dark:hover:bg-stone-800"
            >
                {#if isDark}
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        ><path
                            d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                        ></path></svg
                    >
                {:else}
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        ><circle cx="12" cy="12" r="5"></circle><line
                            x1="12"
                            y1="1"
                            x2="12"
                            y2="3"
                        ></line><line x1="12" y1="21" x2="12" y2="23"
                        ></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"
                        ></line><line
                            x1="18.36"
                            y1="18.36"
                            x2="19.78"
                            y2="19.78"
                        ></line><line x1="1" y1="12" x2="3" y2="12"></line><line
                            x1="21"
                            y1="12"
                            x2="23"
                            y2="12"
                        ></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"
                        ></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"
                        ></line></svg
                    >
                {/if}
            </button>
        </div>
    </div>
</div>

<main class="container mx-auto p-4 max-w-6xl space-y-8 mt-6">
    <div
        class="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-6 px-4"
    >
        <span class="text-7xl opacity-90 drop-shadow-md">🦊</span>
        <div class="space-y-2">
            <h2
                class="text-3xl font-bold text-stone-800 dark:text-stone-200"
            >
                {$t("home.title")}
            </h2>
            <p
                class="text-stone-500 dark:text-stone-400 text-lg max-w-md mx-auto"
            >
                {$t("home.subtitle")}
                <button
                    on:click={() => {
                        searchTerm = "machen";
                        handleSearch();
                    }}
                    class="text-orange-500 hover:text-orange-600 underline underline-offset-4 decoration-orange-200 mx-1 transition-colors"
                    >machen</button
                >,
                <button
                    on:click={() => {
                        searchTerm = "haben";
                        handleSearch();
                    }}
                    class="text-orange-500 hover:text-orange-600 underline underline-offset-4 decoration-orange-200 mx-1 transition-colors"
                    >haben</button
                >
                {$t("home.or")}
                <button
                    on:click={() => {
                        searchTerm = "sein";
                        handleSearch();
                    }}
                    class="text-orange-500 hover:text-orange-600 underline underline-offset-4 decoration-orange-200 mx-1 transition-colors"
                    >sein</button
                >.
            </p>
        </div>
    </div>

    <WordStats />
</main>

<style>
    :global(html) {
        scroll-behavior: smooth;
    }
</style>
