<script lang="ts">
    import WordStats from "$lib/components/WordStats.svelte";
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    export let data: PageData;

    let searchTerm = "";

    $: w = data?.word;
    $: preps = w?.praepositionen_ua || [];

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
                    placeholder="Найти глагол..."
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
                    Картки
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
                    Арена
                </a>

                <a
                    href="/login"
                    class="text-sm font-medium text-stone-600 dark:text-stone-300 hover:text-orange-500 transition-colors"
                >
                    Увійти
                </a>
                <a
                    href="/register"
                    class="text-sm font-bold bg-orange-500 hover:bg-orange-600 text-white px-4 py-2 rounded-lg transition-colors shadow-sm"
                >
                    Реєстрація
                </a>
            </div>
            <button
                on:click={toggleTheme}
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
    {#if w}
        <header
            class="flex flex-col md:flex-row md:items-end justify-between gap-6 pb-6 border-b border-stone-200 dark:border-stone-800"
        >
            <div class="space-y-1">
                <div class="flex items-center gap-3">
                    <h1
                        class="text-4xl md:text-5xl font-extrabold tracking-tight text-stone-900 dark:text-white"
                    >
                        {w.lemma}
                    </h1>
                    <span
                        class="text-xs font-mono text-stone-500 bg-stone-200 dark:bg-stone-800 px-2 py-1 rounded-md mt-2 md:mt-4"
                        >infinitiv</span
                    >
                </div>
                {#if w.translation_ua}
                    <h3
                        class="text-xl md:text-2xl font-medium text-stone-600 dark:text-stone-300 mt-2"
                    >
                        {w.translation_ua}
                    </h3>
                {/if}
            </div>

            <div class="flex items-center gap-3 text-sm">
                <span class="text-stone-500">Hilfsverb:</span>
                <span
                    class="font-mono bg-white dark:bg-stone-900 border border-stone-200 dark:border-stone-700 px-3 py-1 rounded-md text-stone-900 dark:text-stone-100 font-bold shadow-sm"
                >
                    {w.aux ?? "—"}
                </span>
            </div>
        </header>

        {#if preps.length > 0}
            <section
                class="border border-orange-200 dark:border-orange-900/50 rounded-2xl p-6 bg-orange-50/50 dark:bg-orange-900/10"
            >
                <h4
                    class="text-sm font-bold text-orange-600 dark:text-orange-500 uppercase tracking-wider mb-4 flex items-center gap-2"
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
                        ><path
                            d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"
                        ></path><path
                            d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"
                        ></path></svg
                    >
                    Управління (Rektion)
                </h4>
                <div class="flex flex-col gap-3">
                    {#each preps as prep}
                        <div
                            class="flex flex-col md:flex-row md:items-center gap-2 md:gap-4"
                        >
                            <span
                                class="font-mono font-bold bg-white dark:bg-stone-950 border border-stone-200 dark:border-stone-800 text-stone-900 dark:text-stone-100 px-3 py-1.5 rounded-lg text-sm shadow-sm"
                            >
                                {prep.phrase}
                            </span>
                            <span class="text-stone-700 dark:text-stone-300"
                                >{prep.ua}</span
                            >
                        </div>
                    {/each}
                </div>
            </section>
        {/if}

        <div
            class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5 md:gap-6"
        >
            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
            >
                <h4
                    class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                >
                    Präsens
                </h4>
                <ul class="space-y-2 text-sm">
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ich</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs1sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">du</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs2sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">er/sie/es</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs3sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">wir</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs1pl ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ihr</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs2pl ?? "-"}</strong
                        >
                    </li>
                    <li class="flex justify-between">
                        <span class="text-stone-500 w-16">sie/Sie</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prs3pl ?? "-"}</strong
                        >
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
            >
                <h4
                    class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                >
                    Präteritum
                </h4>
                <ul class="space-y-2 text-sm">
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ich</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt1sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">du</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt2sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">er/sie/es</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt3sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">wir</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt1pl ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ihr</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt2pl ?? "-"}</strong
                        >
                    </li>
                    <li class="flex justify-between">
                        <span class="text-stone-500 w-16">sie/Sie</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.prt3pl ?? "-"}</strong
                        >
                    </li>
                </ul>
            </section>

            {#if w.pa2 && w.aux}
                <section
                    class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
                >
                    <h4
                        class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                    >
                        Perfekt
                    </h4>
                    <ul class="space-y-2 text-sm">
                        <li
                            class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                        >
                            <span class="text-stone-500 w-16">ich</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "bin" : "habe"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                        <li
                            class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                        >
                            <span class="text-stone-500 w-16">du</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "bist" : "hast"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                        <li
                            class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                        >
                            <span class="text-stone-500 w-16">er/sie/es</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "ist" : "hat"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                        <li
                            class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                        >
                            <span class="text-stone-500 w-16">wir</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "sind" : "haben"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                        <li
                            class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                        >
                            <span class="text-stone-500 w-16">ihr</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "seid" : "habt"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                        <li class="flex justify-between">
                            <span class="text-stone-500 w-16">sie/Sie</span>
                            <span class="text-stone-900 dark:text-stone-100"
                                >{w.aux === "sein" ? "sind" : "haben"}
                                <strong>{w.pa2}</strong></span
                            >
                        </li>
                    </ul>
                </section>
            {/if}

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
            >
                <h4
                    class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                >
                    Futur I
                </h4>
                <ul class="space-y-2 text-sm">
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ich</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_1sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">du</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_2sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">er/sie/es</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_3sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">wir</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_1pl ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ihr</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_2pl ?? "-"}</strong
                        >
                    </li>
                    <li class="flex justify-between">
                        <span class="text-stone-500 w-16">sie/Sie</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.fut1_3pl ?? "-"}</strong
                        >
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
            >
                <h4
                    class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                >
                    Konjunktiv I
                </h4>
                <ul class="space-y-2 text-sm">
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ich</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_1sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">du</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_2sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">er/sie/es</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_3sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">wir</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_1pl ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ihr</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_2pl ?? "-"}</strong
                        >
                    </li>
                    <li class="flex justify-between">
                        <span class="text-stone-500 w-16">sie/Sie</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj1_3pl ?? "-"}</strong
                        >
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-5 shadow-sm"
            >
                <h4
                    class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-4"
                >
                    Konjunktiv II
                </h4>
                <ul class="space-y-2 text-sm">
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ich</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_1sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">du</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_2sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">er/sie/es</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_3sg ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">wir</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_1pl ?? "-"}</strong
                        >
                    </li>
                    <li
                        class="flex justify-between border-b border-stone-100 dark:border-stone-800/50 pb-1"
                    >
                        <span class="text-stone-500 w-16">ihr</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_2pl ?? "-"}</strong
                        >
                    </li>
                    <li class="flex justify-between">
                        <span class="text-stone-500 w-16">sie/Sie</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.kj2_3pl ?? "-"}</strong
                        >
                    </li>
                </ul>
            </section>

            <section
                class="col-span-full bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-6 shadow-sm grid grid-cols-1 md:grid-cols-3 gap-8"
            >
                <div class="space-y-3">
                    <h5
                        class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest border-b border-stone-100 dark:border-stone-800 pb-2"
                    >
                        Imperativ
                    </h5>
                    <p class="flex justify-between text-sm">
                        <span class="text-stone-500">du:</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.imp2sg ?? "-"}</strong
                        >
                    </p>
                    <p class="flex justify-between text-sm">
                        <span class="text-stone-500">ihr:</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.imp2pl ?? "-"}</strong
                        >
                    </p>
                </div>
                <div class="space-y-3">
                    <h5
                        class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest border-b border-stone-100 dark:border-stone-800 pb-2"
                    >
                        Partizipien
                    </h5>
                    <p class="flex justify-between text-sm">
                        <span class="text-stone-500">P I:</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.pa1 ?? "-"}</strong
                        >
                    </p>
                    <p class="flex justify-between text-sm">
                        <span class="text-stone-500">P II:</span>
                        <strong class="text-stone-900 dark:text-stone-100"
                            >{w.pa2 ?? "-"}</strong
                        >
                    </p>
                </div>
                <div
                    class="flex flex-col justify-center items-center p-4 bg-stone-50 dark:bg-stone-900 rounded-xl border border-stone-200 dark:border-stone-800"
                >
                    <h5
                        class="text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-2"
                    >
                        Infinitive
                    </h5>
                    <strong
                        class="text-xl font-bold text-stone-900 dark:text-stone-200"
                        >{w.inf ?? "-"}</strong
                    >
                </div>
            </section>
        </div>
    {:else}
        <div
            class="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-6 px-4"
        >
            <span class="text-7xl opacity-90 drop-shadow-md">🦊</span>
            <div class="space-y-2">
                <h2
                    class="text-3xl font-bold text-stone-800 dark:text-stone-200"
                >
                    Введіть дієслово в пошук
                </h2>
                <p
                    class="text-stone-500 dark:text-stone-400 text-lg max-w-md mx-auto"
                >
                    Знайдіть відмінювання, переклад та керування німецьких
                    дієслів. Наприклад:
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
                    або
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
    {/if}
</main>

<style>
    :global(html) {
        scroll-behavior: smooth;
    }
</style>
