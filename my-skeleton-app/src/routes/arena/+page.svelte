<script lang="ts">
    import { browser } from "$app/environment";
    import { onDestroy } from "svelte";

    type Question = {
        index: number;
        lemma: string;
        options: string[];
    };

    type GameStatus = "idle" | "waiting" | "playing" | "finished";

    let ws: WebSocket | null = null;
    let status: GameStatus = "idle";
    let opponent = "";
    let questions: Question[] = [];
    let currentIndex = 0;
    let yourScore = 0;
    let opponentScore = 0;
    let answered = 0;
    let selectedAnswer: string | null = null;
    let lastCorrect: boolean | null = null;
    let finishResult: "win" | "lose" | "draw" | null = null;

    function connect() {
        if (!browser) return;
        const targetIp = window.location.hostname;
        ws = new WebSocket(`ws://${targetIp}:8080/api/ws/match`);

        ws.onopen = () => {
            status = "waiting";
        };

        ws.onmessage = (event) => {
            const msg = JSON.parse(event.data);

            if (msg.action === "start") {
                opponent = msg.opponent;
                questions = msg.questions;
                currentIndex = 0;
                yourScore = 0;
                answered = 0;
                selectedAnswer = null;
                lastCorrect = null;
                status = "playing";
            }

            if (msg.action === "score_update") {
                yourScore = msg.your_score;
                answered = msg.answered;
                setTimeout(() => {
                    selectedAnswer = null;
                    lastCorrect = null;
                    if (currentIndex < questions.length - 1) {
                        currentIndex += 1;
                    }
                }, 800);
            }

            if (msg.action === "finish") {
                finishResult = msg.result;
                yourScore = msg.your_score;
                opponentScore = msg.opponent_score;
                status = "finished";
            }
        };

        ws.onclose = () => {
            if (status !== "finished") {
                status = "idle";
            }
        };
    }

    function sendAnswer(option: string) {
        if (!ws || selectedAnswer !== null) return;
        const q = questions[currentIndex];
        selectedAnswer = option;
        ws.send(JSON.stringify({ index: q.index, answer: option }));
    }

    function reset() {
        ws?.close();
        ws = null;
        status = "idle";
        finishResult = null;
        questions = [];
        currentIndex = 0;
        yourScore = 0;
        opponentScore = 0;
        selectedAnswer = null;
        lastCorrect = null;
    }

    onDestroy(() => ws?.close());

    $: currentQuestion = questions[currentIndex] ?? null;
    $: progress =
        questions.length > 0 ? (currentIndex / questions.length) * 100 : 0;
</script>

<div class="max-w-md mx-auto py-12 px-4 space-y-6">
    <div class="flex justify-start">
        <a
            href="/"
            class="flex items-center gap-2 text-sm font-medium text-stone-500 hover:text-orange-500 transition-colors"
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
            >
                <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                <polyline points="9 22 9 12 15 12 15 22"></polyline>
            </svg>
            На головну
        </a>
    </div>

    <div class="text-center">
        <h1 class="text-3xl font-extrabold text-stone-900 dark:text-white">
            Арена
        </h1>
        <p class="text-stone-500">Змагайтеся з іншими на швидкість</p>
    </div>

    {#if status === "idle"}
        <button
            on:click={connect}
            class="w-full bg-orange-500 hover:bg-orange-600 text-white font-bold py-3 rounded-xl transition-colors"
        >
            Знайти суперника
        </button>
    {:else if status === "waiting"}
        <div class="text-center space-y-4">
            <div class="text-4xl animate-pulse">⏳</div>
            <p class="text-stone-600 dark:text-stone-400">
                Шукаємо суперника...
            </p>
            <button on:click={reset} class="text-sm text-stone-400 underline"
                >Скасувати</button
            >
        </div>
    {:else if status === "playing" && currentQuestion}
        <div
            class="flex justify-between items-center text-sm text-stone-500 dark:text-stone-400"
        >
            <span
                >vs <strong class="text-stone-700 dark:text-stone-200"
                    >{opponent}</strong
                ></span
            >
            <span>✅ {yourScore} / {questions.length}</span>
        </div>

        <div class="w-full bg-stone-200 dark:bg-stone-700 rounded-full h-2">
            <div
                class="bg-orange-500 h-2 rounded-full transition-all duration-300"
                style="width: {progress}%"
            ></div>
        </div>

        <div class="text-center py-6">
            <p class="text-xs uppercase tracking-widest text-stone-400 mb-2">
                Питання {currentIndex + 1} з {questions.length}
            </p>
            <p class="text-4xl font-extrabold text-stone-900 dark:text-white">
                {currentQuestion.lemma}
            </p>
        </div>

        <div class="grid grid-cols-2 gap-3">
            {#each currentQuestion.options as option}
                {@const isSelected = selectedAnswer === option}
                <button
                    on:click={() => sendAnswer(option)}
                    disabled={selectedAnswer !== null}
                    class="
                        py-4 px-3 rounded-xl font-semibold text-sm border-2 transition-all duration-200
                        {selectedAnswer === null
                        ? 'border-stone-200 dark:border-stone-700 bg-white dark:bg-stone-900 hover:border-orange-400 hover:bg-orange-50 dark:hover:bg-stone-800 text-stone-800 dark:text-stone-200'
                        : isSelected
                          ? 'border-orange-500 bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300'
                          : 'border-stone-100 dark:border-stone-800 bg-stone-50 dark:bg-stone-950 text-stone-400 dark:text-stone-600'}
                    "
                >
                    {option}
                </button>
            {/each}
        </div>
    {:else if status === "finished"}
        <div class="text-center space-y-6">
            <div class="text-6xl">
                {#if finishResult === "win"}🏆
                {:else if finishResult === "lose"}💀
                {:else}🤝{/if}
            </div>

            <div>
                <p
                    class="text-2xl font-extrabold text-stone-900 dark:text-white"
                >
                    {#if finishResult === "win"}Ви перемогли!
                    {:else if finishResult === "lose"}Ви програли
                    {:else}Нічия!{/if}
                </p>
                <p class="text-stone-500 mt-1">vs {opponent}</p>
            </div>

            <div class="flex justify-center gap-8">
                <div class="text-center">
                    <p class="text-3xl font-bold text-orange-500">
                        {yourScore}
                    </p>
                    <p class="text-xs text-stone-400 mt-1">Ваш рахунок</p>
                </div>
                <div class="text-center opacity-50">
                    <p class="text-3xl font-bold text-stone-500">
                        {opponentScore}
                    </p>
                    <p class="text-xs text-stone-400 mt-1">{opponent}</p>
                </div>
            </div>

            <button
                on:click={connect}
                class="w-full bg-orange-500 hover:bg-orange-600 text-white font-bold py-3 rounded-xl transition-colors"
            >
                Грати знову
            </button>
            <button
                on:click={reset}
                class="text-sm text-stone-400 underline block w-full"
            >
                На головну
            </button>
        </div>
    {/if}
</div>
