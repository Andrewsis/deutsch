<script lang="ts">
    import { goto } from "$app/navigation";
    import { browser } from "$app/environment";

    let showPassword = false;

    let username = "";
    let password = "";
    let errorMessage = "";
    let isLoading = false;

    function handlePasswordInput(e: Event) {
        password = (e.target as HTMLInputElement).value;
    }

    async function handleLogin() {
        errorMessage = "";
        isLoading = true;

        const targetIp = browser ? window.location.hostname : "127.0.0.1";

        try {
            const res = await fetch(`http://${targetIp}:8080/api/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({ username, password }),
            });

            if (!res.ok) {
                const errorData = await res.text();
                errorMessage = errorData || "Невірний логін або пароль";
            } else {
                window.location.href = "/";
            }
        } catch (e) {
            console.error(e);
            errorMessage = "Не вдалося з'єднатися з сервером";
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="min-h-[80vh] flex items-center justify-center px-4 py-12">
    <div
        class="w-full max-w-md bg-white dark:bg-[#1a1a1a] border border-stone-200 dark:border-stone-800 rounded-2xl p-6 sm:p-8 shadow-sm"
    >
        <div class="text-center mb-8">
            <h1
                class="text-3xl font-extrabold tracking-tight text-stone-900 dark:text-white mb-2"
            >
                Вхід
            </h1>
            <p class="text-stone-500 dark:text-stone-400">
                З поверненням! Увійдіть у свій акаунт.
            </p>
        </div>

        <form on:submit|preventDefault={handleLogin} class="space-y-5">
            {#if errorMessage}
                <div
                    class="p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/10 dark:text-red-400 rounded-lg border border-red-200 dark:border-red-900/50"
                >
                    {errorMessage}
                </div>
            {/if}

            <div>
                <label
                    for="username"
                    class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5"
                    >Ім'я користувача</label
                >
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    required
                    class="w-full bg-stone-100 dark:bg-stone-900 border border-transparent focus:border-orange-500 focus:bg-white dark:focus:bg-stone-950 focus:ring-0 rounded-lg px-4 py-2.5 text-sm transition-all outline-none text-stone-900 dark:text-stone-100 placeholder-stone-500"
                    placeholder="john_doe"
                />
            </div>

            <div>
                <label
                    for="password"
                    class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5"
                    >Пароль</label
                >
                <div class="relative">
                    <input
                        id="password"
                        type={showPassword ? "text" : "password"}
                        value={password}
                        on:input={handlePasswordInput}
                        required
                        class="w-full bg-stone-100 dark:bg-stone-900 border border-transparent focus:border-orange-500 focus:bg-white dark:focus:bg-stone-950 focus:ring-0 rounded-lg pl-4 pr-10 py-2.5 text-sm transition-all outline-none text-stone-900 dark:text-stone-100 placeholder-stone-500"
                        placeholder="••••••••"
                    />
                    <button
                        type="button"
                        class="absolute right-3 top-1/2 -translate-y-1/2 text-stone-400 hover:text-stone-600 dark:hover:text-stone-300 transition-colors"
                        on:click={() => (showPassword = !showPassword)}
                        aria-label={showPassword
                            ? "Сховати пароль"
                            : "Показати пароль"}
                    >
                        {#if showPassword}
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
                            >
                                <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path>
                                <path
                                    d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"
                                ></path>
                                <path
                                    d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"
                                ></path>
                                <line x1="2" x2="22" y1="2" y2="22"></line>
                            </svg>
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
                            >
                                <path
                                    d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"
                                ></path>
                                <circle cx="12" cy="12" r="3"></circle>
                            </svg>
                        {/if}
                    </button>
                </div>
            </div>

            <button
                type="submit"
                disabled={isLoading}
                class="w-full mt-2 bg-stone-900 hover:bg-stone-800 dark:bg-white dark:hover:bg-stone-200 dark:text-stone-900 text-white font-bold py-2.5 px-4 rounded-lg transition-colors flex justify-center items-center gap-2 shadow-sm"
            >
                {#if isLoading}
                    <svg
                        class="animate-spin -ml-1 mr-2 h-4 w-4 text-current"
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
                {/if}
                Увійти
            </button>
        </form>

        <div
            class="mt-6 text-center text-sm text-stone-500 dark:text-stone-400"
        >
            Немає акаунту?
            <a
                href="/register"
                class="text-orange-500 hover:text-orange-600 underline underline-offset-4 decoration-orange-200 dark:decoration-orange-900 transition-colors font-medium"
                >Зареєструватися</a
            >
        </div>
    </div>
</div>
