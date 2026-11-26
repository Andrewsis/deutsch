<script lang="ts">
    import { onMount } from "svelte";
    import { apiUrl, withLang } from "$lib/api";
    import { t } from "$lib/i18n";

    let flashcards: any[] = [];
    let currentIndex = 0;
    let isFlipped = false;
    let isLoading = true;
    let errorKey: "study.loginRequired" | "study.loadError" | "common.noConnection" | null = null;

    $: currentWord = flashcards[currentIndex];

    onMount(async () => {

        try {
            const res = await fetch(
                apiUrl(withLang("/api/flashcards/due")),
                {
                    credentials: "include",
                },
            );

            if (res.ok) {
                flashcards = await res.json();
            } else if (res.status === 401) {
                errorKey = "study.loginRequired";
            } else {
                errorKey = "study.loadError";
            }
        } catch (e) {
            console.error(e);
            errorKey = "common.noConnection";
        } finally {
            isLoading = false;
        }
    });

    async function submitReview(quality: number) {

        try {
            const res = await fetch(
                apiUrl("/api/flashcards/review"),
                {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    credentials: "include",
                    body: JSON.stringify({
                        verb_id: currentWord.id,
                        quality: quality,
                    }),
                },
            );

            if (!res.ok) {
                const errText = await res.text();
                console.error(`Server error (${res.status}):`, errText);
            } else {
                console.log("Review saved");
            }
        } catch (e) {
            console.error("Network error (CORS or server down):", e);
        }

        isFlipped = false;
        currentIndex++;
    }
</script>

<div class="max-w-xl mx-auto py-12 px-4 flex flex-col items-center">
    {#if isLoading}
        <div class="flex flex-col items-center justify-center h-80 space-y-4">
            <svg
                class="animate-spin h-8 w-8 text-orange-500"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                ></circle>
                <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
            </svg>
            <p class="text-stone-500">{$t("study.loading")}</p>
        </div>
    {:else if errorKey}
        <div
            class="flex flex-col items-center justify-center h-80 text-center space-y-4"
        >
            <span class="text-5xl">🔒</span>
            <h2 class="text-xl font-bold text-stone-900 dark:text-white">
                {$t(errorKey)}
            </h2>
            {#if errorKey === "study.loginRequired"}
                <a
                    href="/login"
                    class="px-6 py-2 bg-orange-500 hover:bg-orange-600 text-white font-bold rounded-lg transition-colors mt-2"
                    >{$t("common.login")}</a
                >
            {/if}
        </div>
    {:else if !currentWord}
        <div
            class="flex flex-col items-center justify-center h-80 text-center space-y-4"
        >
            <span class="text-6xl drop-shadow-md">🎉</span>
            <h2 class="text-2xl font-bold text-stone-900 dark:text-white">
                {$t("study.done")}
            </h2>
            <p class="text-stone-500">{$t("study.doneText")}</p>
            <a
                href="/"
                class="px-6 py-2 bg-stone-900 dark:bg-white text-white dark:text-stone-900 font-bold rounded-lg transition-colors mt-4"
                >{$t("common.backHome")}</a
            >
        </div>
    {:else}
        <div
            class="w-full mb-4 flex justify-between text-sm font-bold text-stone-400"
        >
            <span>{$t("study.progress")}</span>
            <span>{currentIndex + 1} / {flashcards.length}</span>
        </div>

        <div
            class="w-full h-80 bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-3xl shadow-sm cursor-pointer relative"
            on:click={() => !isFlipped && (isFlipped = true)}
        >
            {#if !isFlipped}
                <div
                    class="absolute inset-0 flex flex-col items-center justify-center p-8 text-center"
                >
                    <span
                        class="text-stone-400 font-mono text-sm uppercase tracking-widest mb-4"
                        >Infinitiv</span
                    >
                    <h2
                        class="text-5xl font-extrabold text-stone-900 dark:text-white"
                    >
                        {currentWord.lemma}
                    </h2>
                    <p class="text-stone-400 mt-6 text-sm">
                        {$t("study.flip")}
                    </p>
                </div>
            {:else}
                <div
                    class="absolute inset-0 flex flex-col items-center justify-center p-8 text-center bg-stone-50 dark:bg-stone-900/50 rounded-3xl"
                >
                    <h3
                        class="text-3xl font-bold text-orange-600 dark:text-orange-500 mb-2"
                    >
                        {currentWord.translation || "—"}
                    </h3>

                    <div
                        class="grid grid-cols-2 gap-x-8 gap-y-4 mt-6 text-left w-full max-w-sm"
                    >
                        <div class="flex flex-col">
                            <span class="text-xs text-stone-500 uppercase"
                                >Präsens (er/sie/es)</span
                            >
                            <span class="font-bold dark:text-white"
                                >{currentWord.prs3sg || "—"}</span
                            >
                        </div>
                        <div class="flex flex-col">
                            <span class="text-xs text-stone-500 uppercase"
                                >Präteritum</span
                            >
                            <span class="font-bold dark:text-white"
                                >{currentWord.prt3sg || "—"}</span
                            >
                        </div>
                        <div
                            class="flex flex-col col-span-2 border-t border-stone-200 dark:border-stone-700 pt-3"
                        >
                            <span class="text-xs text-stone-500 uppercase"
                                >Perfekt</span
                            >
                            <span class="font-bold dark:text-white">
                                {currentWord.aux
                                    ? currentWord.aux + " "
                                    : ""}{currentWord.pa2 || "—"}
                            </span>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        {#if isFlipped}
            <div class="flex justify-between w-full gap-2 mt-8">
                <button
                    on:click={() => submitReview(0)}
                    class="flex-1 bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-900/30 dark:text-red-400 py-3 rounded-xl font-bold transition-colors"
                    >{$t("study.again")}</button
                >
                <button
                    on:click={() => submitReview(1)}
                    class="flex-1 bg-orange-100 text-orange-700 hover:bg-orange-200 dark:bg-orange-900/30 dark:text-orange-400 py-3 rounded-xl font-bold transition-colors"
                    >{$t("study.hard")}</button
                >
                <button
                    on:click={() => submitReview(2)}
                    class="flex-1 bg-green-100 text-green-700 hover:bg-green-200 dark:bg-green-900/30 dark:text-green-400 py-3 rounded-xl font-bold transition-colors"
                    >{$t("study.good")}</button
                >
                <button
                    on:click={() => submitReview(3)}
                    class="flex-1 bg-blue-100 text-blue-700 hover:bg-blue-200 dark:bg-blue-900/30 dark:text-blue-400 py-3 rounded-xl font-bold transition-colors"
                    >{$t("study.easy")}</button
                >
            </div>
        {/if}
    {/if}
</div>
