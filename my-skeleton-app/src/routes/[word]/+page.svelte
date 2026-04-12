<script lang="ts">
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";

    export let data: PageData;

    let searchTerm = "";

    $: w = data?.word;
    $: preps = w?.praepositionen_ua || [];

    $: audioForms = w?.audio_forms || [];

    function hasAudio(formText: string | null | undefined): boolean {
        if (!formText) return false;
        return audioForms.includes(formText);
    }

    let isDark = false;

    let isPlayerVisible = false;
    let isLoadingVideo = false;
    let currentExamples: any[] = [];
    let currentIndex = 0;
    let targetWord = "";

    $: currentVideo = currentExamples[currentIndex];

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

    async function playAudio(formText: string) {
        targetWord = formText;
        isLoadingVideo = true;
        isPlayerVisible = true;
        currentExamples = [];
        currentIndex = 0;

        const targetIp = browser ? "192.168.1.71" : "127.0.0.1";

        try {
            const res = await fetch(
                `http://${targetIp}:8080/api/examples?word=${encodeURIComponent(formText)}&limit=10&offset=0`,
            );
            if (res.ok) {
                currentExamples = await res.json();
            } else {
                console.error("Помилка завантаження відео");
            }
        } catch (e) {
            console.error(e);
        } finally {
            isLoadingVideo = false;
        }
    }

    function closePlayer() {
        isPlayerVisible = false;
        currentExamples = [];
    }

    function nextVideo() {
        if (currentIndex < currentExamples.length - 1) currentIndex++;
    }

    function prevVideo() {
        if (currentIndex > 0) currentIndex--;
    }

    function highlightText(text: string, word: string) {
        if (!text || !word) return text;
        const regex = new RegExp(`(${word})`, "gi");
        return text.replace(
            regex,
            '<span class="bg-orange-200 dark:bg-orange-900 text-orange-900 dark:text-orange-100 font-bold px-1 rounded">$1</span>',
        );
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

<main
    class="container mx-auto p-4 max-w-6xl space-y-6 md:space-y-8 mt-2 md:mt-6"
>
    {#if w}
        <header
            class="flex flex-col md:flex-row md:items-end justify-between gap-4 md:gap-6 pb-4 md:pb-6 border-b border-stone-200 dark:border-stone-800"
        >
            <div class="space-y-1">
                <div class="flex items-center gap-3">
                    <h1
                        class="text-3xl md:text-5xl font-extrabold tracking-tight text-stone-900 dark:text-white"
                    >
                        {w.lemma}
                    </h1>
                    <span
                        class="text-[10px] md:text-xs font-mono text-stone-500 bg-stone-200 dark:bg-stone-800 px-2 py-1 rounded-md mt-1 md:mt-4"
                        >infinitiv</span
                    >
                </div>
                {#if w.translation_ua}
                    <h3
                        class="text-lg md:text-2xl font-medium text-stone-600 dark:text-stone-300 mt-1 md:mt-2"
                    >
                        {w.translation_ua}
                    </h3>
                {/if}
            </div>

            <div class="flex items-center gap-3 text-xs md:text-sm">
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
                class="border border-orange-200 dark:border-orange-900/50 rounded-xl md:rounded-2xl p-3 md:p-6 bg-orange-50/50 dark:bg-orange-900/10"
            >
                <h4
                    class="text-[11px] md:text-sm font-bold text-orange-600 dark:text-orange-500 uppercase tracking-wider mb-2 md:mb-4 flex items-center gap-2"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="md:w-4 md:h-4"
                        ><path
                            d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"
                        ></path><path
                            d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"
                        ></path></svg
                    >
                    Управління (Rektion)
                </h4>
                <div class="flex flex-col gap-2 md:gap-3">
                    {#each preps as prep}
                        <div
                            class="flex flex-col sm:flex-row sm:items-center gap-1 sm:gap-4"
                        >
                            <span
                                class="font-mono font-bold bg-white dark:bg-stone-950 border border-stone-200 dark:border-stone-800 text-stone-900 dark:text-stone-100 px-2 md:px-3 py-1 md:py-1.5 rounded-lg text-xs md:text-sm shadow-sm inline-block w-max"
                            >
                                {prep.phrase}
                            </span>
                            <span
                                class="text-stone-700 dark:text-stone-300 text-[13px] md:text-base leading-tight"
                                >{prep.ua}</span
                            >
                        </div>
                    {/each}
                </div>
            </section>
        {/if}

        <div class="grid grid-cols-2 lg:grid-cols-3 gap-2 md:gap-6">
            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
            >
                <h4
                    class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                >
                    Präsens
                </h4>
                <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ich</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs1sg)}<button
                                    on:click={() => playAudio(w.prs1sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs1sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">du</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs2sg)}<button
                                    on:click={() => playAudio(w.prs2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs2sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">er/sie/es</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs3sg)}<button
                                    on:click={() => playAudio(w.prs3sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs3sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">wir</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs1pl)}<button
                                    on:click={() => playAudio(w.prs1pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs1pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ihr</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs2pl)}<button
                                    on:click={() => playAudio(w.prs2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs2pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li class="group flex justify-between items-center">
                        <span class="text-stone-500 shrink-0">sie/Sie</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prs3pl)}<button
                                    on:click={() => playAudio(w.prs3pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prs3pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
            >
                <h4
                    class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                >
                    Präteritum
                </h4>
                <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ich</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt1sg)}<button
                                    on:click={() => playAudio(w.prt1sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt1sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">du</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt2sg)}<button
                                    on:click={() => playAudio(w.prt2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt2sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">er/sie/es</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt3sg)}<button
                                    on:click={() => playAudio(w.prt3sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt3sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">wir</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt1pl)}<button
                                    on:click={() => playAudio(w.prt1pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt1pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ihr</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt2pl)}<button
                                    on:click={() => playAudio(w.prt2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt2pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li class="group flex justify-between items-center">
                        <span class="text-stone-500 shrink-0">sie/Sie</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.prt3pl)}<button
                                    on:click={() => playAudio(w.prt3pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.prt3pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                </ul>
            </section>

            {#if w.pa2 && w.aux}
                <section
                    class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
                >
                    <h4
                        class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                    >
                        Perfekt
                    </h4>
                    <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                        <li
                            class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                        >
                            <span class="text-stone-500 shrink-0">ich</span>
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "bin" : "habe"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                        <li
                            class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                        >
                            <span class="text-stone-500 shrink-0">du</span>
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "bist" : "hast"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                        <li
                            class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                        >
                            <span class="text-stone-500 shrink-0"
                                >er/sie/es</span
                            >
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "ist" : "hat"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                        <li
                            class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                        >
                            <span class="text-stone-500 shrink-0">wir</span>
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "sind" : "haben"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                        <li
                            class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                        >
                            <span class="text-stone-500 shrink-0">ihr</span>
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "seid" : "habt"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                        <li class="group flex justify-between items-center">
                            <span class="text-stone-500 shrink-0">sie/Sie</span>
                            <div class="flex items-center gap-2">
                                {#if hasAudio(w.pa2)}<button
                                        on:click={() => playAudio(w.pa2)}
                                        class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                        title="Слухати">🔊</button
                                    >{/if}
                                <span class="text-stone-900 dark:text-stone-100"
                                    >{w.aux === "sein" ? "sind" : "haben"}
                                    <strong>{w.pa2}</strong></span
                                >
                            </div>
                        </li>
                    </ul>
                </section>
            {/if}

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
            >
                <h4
                    class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                >
                    Futur I
                </h4>
                <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ich</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_1sg)}<button
                                    on:click={() => playAudio(w.fut1_1sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_1sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">du</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_2sg)}<button
                                    on:click={() => playAudio(w.fut1_2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_2sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">er/sie/es</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_3sg)}<button
                                    on:click={() => playAudio(w.fut1_3sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_3sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">wir</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_1pl)}<button
                                    on:click={() => playAudio(w.fut1_1pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_1pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ihr</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_2pl)}<button
                                    on:click={() => playAudio(w.fut1_2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_2pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li class="group flex justify-between items-center">
                        <span class="text-stone-500 shrink-0">sie/Sie</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.fut1_3pl)}<button
                                    on:click={() => playAudio(w.fut1_3pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.fut1_3pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
            >
                <h4
                    class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                >
                    Konjunktiv I
                </h4>
                <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ich</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_1sg)}<button
                                    on:click={() => playAudio(w.kj1_1sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_1sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">du</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_2sg)}<button
                                    on:click={() => playAudio(w.kj1_2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_2sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">er/sie/es</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_3sg)}<button
                                    on:click={() => playAudio(w.kj1_3sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_3sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">wir</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_1pl)}<button
                                    on:click={() => playAudio(w.kj1_1pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_1pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ihr</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_2pl)}<button
                                    on:click={() => playAudio(w.kj1_2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_2pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li class="group flex justify-between items-center">
                        <span class="text-stone-500 shrink-0">sie/Sie</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj1_3pl)}<button
                                    on:click={() => playAudio(w.kj1_3pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj1_3pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                </ul>
            </section>

            <section
                class="bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-2.5 md:p-5 shadow-sm"
            >
                <h4
                    class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-1.5 md:mb-4"
                >
                    Konjunktiv II
                </h4>
                <ul class="space-y-1 md:space-y-2 text-[13px] md:text-sm">
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ich</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_1sg)}<button
                                    on:click={() => playAudio(w.kj2_1sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_1sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">du</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_2sg)}<button
                                    on:click={() => playAudio(w.kj2_2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_2sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">er/sie/es</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_3sg)}<button
                                    on:click={() => playAudio(w.kj2_3sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_3sg ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">wir</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_1pl)}<button
                                    on:click={() => playAudio(w.kj2_1pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_1pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li
                        class="group flex justify-between items-center border-b border-stone-100 dark:border-stone-800/50 pb-0.5 md:pb-1"
                    >
                        <span class="text-stone-500 shrink-0">ihr</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_2pl)}<button
                                    on:click={() => playAudio(w.kj2_2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_2pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                    <li class="group flex justify-between items-center">
                        <span class="text-stone-500 shrink-0">sie/Sie</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.kj2_3pl)}<button
                                    on:click={() => playAudio(w.kj2_3pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.kj2_3pl ?? "-"}</strong
                            >
                        </div>
                    </li>
                </ul>
            </section>

            <section
                class="col-span-full bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl md:rounded-2xl p-3 md:p-6 shadow-sm grid grid-cols-2 lg:grid-cols-3 gap-3 md:gap-8"
            >
                <div class="space-y-1.5 md:space-y-3">
                    <h5
                        class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest border-b border-stone-100 dark:border-stone-800 pb-1 md:pb-2"
                    >
                        Imperativ
                    </h5>
                    <div
                        class="flex flex-col md:flex-row md:justify-between text-[13px] md:text-sm gap-0 group"
                    >
                        <span class="text-stone-500">du:</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.imp2sg)}<button
                                    on:click={() => playAudio(w.imp2sg)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.imp2sg ?? "-"}</strong
                            >
                        </div>
                    </div>
                    <div
                        class="flex flex-col md:flex-row md:justify-between text-[13px] md:text-sm gap-0 group"
                    >
                        <span class="text-stone-500">ihr:</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.imp2pl)}<button
                                    on:click={() => playAudio(w.imp2pl)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.imp2pl ?? "-"}</strong
                            >
                        </div>
                    </div>
                </div>

                <div class="space-y-1.5 md:space-y-3">
                    <h5
                        class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest border-b border-stone-100 dark:border-stone-800 pb-1 md:pb-2"
                    >
                        Partizipien
                    </h5>
                    <div
                        class="flex flex-col md:flex-row md:justify-between text-[13px] md:text-sm gap-0 group"
                    >
                        <span class="text-stone-500">P I:</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.pa1)}<button
                                    on:click={() => playAudio(w.pa1)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.pa1 ?? "-"}</strong
                            >
                        </div>
                    </div>
                    <div
                        class="flex flex-col md:flex-row md:justify-between text-[13px] md:text-sm gap-0 group"
                    >
                        <span class="text-stone-500">P II:</span>
                        <div class="flex items-center gap-2">
                            {#if hasAudio(w.pa2)}<button
                                    on:click={() => playAudio(w.pa2)}
                                    class="text-orange-400/70 hover:text-orange-600 dark:hover:text-orange-400 text-base leading-none transition-all hover:scale-110"
                                    title="Слухати">🔊</button
                                >{/if}
                            <strong class="text-stone-900 dark:text-stone-100"
                                >{w.pa2 ?? "-"}</strong
                            >
                        </div>
                    </div>
                </div>

                <div
                    class="col-span-2 lg:col-span-1 flex flex-row lg:flex-col justify-between lg:justify-center items-center p-2.5 md:p-4 bg-stone-50 dark:bg-stone-900 rounded-lg md:rounded-xl border border-stone-200 dark:border-stone-800"
                >
                    <h5
                        class="text-[10px] md:text-xs font-bold text-stone-400 dark:text-stone-500 uppercase tracking-widest mb-0 lg:mb-2"
                    >
                        Infinitive
                    </h5>
                    <div class="flex items-center gap-2 group">
                        {#if hasAudio(w.inf)}<button
                                on:click={() => playAudio(w.inf)}
                                class="opacity-0 group-hover:opacity-100 text-xl leading-none transition-opacity hover:scale-110"
                                title="Слухати">🔊</button
                            >{/if}
                        <strong
                            class="text-[15px] md:text-xl font-bold text-stone-900 dark:text-stone-200"
                            >{w.inf ?? "-"}</strong
                        >
                    </div>
                </div>
            </section>
        </div>
    {:else}
        <div
            class="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-4 md:space-y-6 px-4"
        >
            <span class="text-5xl md:text-7xl opacity-90 drop-shadow-md"
                >🦊</span
            >
            <div class="space-y-2">
                <h2
                    class="text-2xl md:text-3xl font-bold text-stone-800 dark:text-stone-200"
                >
                    Введіть дієслово
                </h2>
                <p
                    class="text-stone-500 dark:text-stone-400 text-sm md:text-lg max-w-md mx-auto"
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
    {/if}
</main>

{#if isPlayerVisible}
    <div
        class="fixed bottom-4 right-4 w-[340px] md:w-[400px] bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-xl shadow-2xl z-50 overflow-hidden flex flex-col transition-all duration-300"
    >
        <div
            class="flex justify-between items-center p-3 bg-stone-50 dark:bg-[#222] border-b border-stone-200 dark:border-stone-800"
        >
            <span class="font-bold text-sm text-stone-800 dark:text-stone-200">
                Приклади з "{targetWord}"
            </span>
            <button
                on:click={closePlayer}
                class="text-stone-500 hover:text-red-500 text-lg leading-none outline-none"
            >
                ✖
            </button>
        </div>

        <div
            class="bg-black aspect-video relative flex items-center justify-center"
        >
            {#if isLoadingVideo}
                <div class="flex flex-col items-center gap-2">
                    <div
                        class="w-5 h-5 border-2 border-orange-500 border-t-transparent rounded-full animate-spin"
                    ></div>
                    <span class="text-white text-xs">Завантаження...</span>
                </div>
            {:else if currentVideo}
                {#key currentVideo.video_id + currentVideo.start_sec}
                    <iframe
                        class="w-full h-full"
                        src="https://www.youtube.com/embed/{currentVideo.video_id}?start={Math.max(
                            0,
                            Math.floor(currentVideo.start_sec) - 1,
                        )}&autoplay=1"
                        frameborder="0"
                        allow="autoplay; encrypted-media"
                        allowfullscreen
                    >
                    </iframe>
                {/key}
            {:else}
                <span class="text-stone-400 text-sm">Відео не знайдено</span>
            {/if}
        </div>

        {#if currentVideo && !isLoadingVideo}
            <div class="p-4 flex flex-col gap-3">
                <p
                    class="text-sm text-stone-800 dark:text-stone-200 leading-relaxed max-h-24 overflow-y-auto"
                >
                    {@html highlightText(currentVideo.full_text, targetWord)}
                </p>

                <div
                    class="flex justify-between items-center pt-2 border-t border-stone-100 dark:border-stone-800"
                >
                    <button
                        on:click={prevVideo}
                        disabled={currentIndex === 0}
                        class="px-2.5 py-1 text-xs rounded bg-stone-100 dark:bg-stone-800 disabled:opacity-30 dark:text-white transition-colors hover:bg-stone-200 dark:hover:bg-stone-700 font-medium"
                    >
                        ← Пред
                    </button>

                    <span
                        class="text-[10px] md:text-xs text-stone-500 font-mono"
                    >
                        {currentIndex + 1} з {currentExamples.length}
                    </span>

                    <button
                        on:click={nextVideo}
                        disabled={currentIndex === currentExamples.length - 1}
                        class="px-2.5 py-1 text-xs rounded bg-stone-100 dark:bg-stone-800 disabled:opacity-30 dark:text-white transition-colors hover:bg-stone-200 dark:hover:bg-stone-700 font-medium"
                    >
                        Слід →
                    </button>
                </div>
            </div>
        {/if}
    </div>
{/if}

<style>
    :global(html) {
        scroll-behavior: smooth;
    }
</style>
