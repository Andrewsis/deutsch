<script lang="ts">
    import { goto } from "$app/navigation";
    import { browser } from "$app/environment";

    let showPassword = false;

    let username = "";
    let email = "";
    let password = "";
    let errorMessage = "";
    let isLoading = false;

    async function handleRegister() {
        errorMessage = "";
        isLoading = true;

        const targetIp = browser ? window.location.hostname : "127.0.0.1";

        try {
            const res = await fetch(`http://${targetIp}:8080/api/register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({ username, email, password }),
            });

            if (!res.ok) {
                const errorData = await res.text();
                errorMessage =
                    errorData || "Помилка реєстрації. Спробуйте ще раз.";
            } else {
                goto("/");
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
                Реєстрація
            </h1>
            <p class="text-stone-500 dark:text-stone-400">
                Створіть акаунт, щоб зберігати свій прогрес
            </p>
        </div>

        <form on:submit|preventDefault={handleRegister} class="space-y-5">
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
                    for="email"
                    class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5"
                    >Електронна пошта</label
                >
                <input
                    id="email"
                    type="email"
                    bind:value={email}
                    required
                    class="w-full bg-stone-100 dark:bg-stone-900 border border-transparent focus:border-orange-500 focus:bg-white dark:focus:bg-stone-950 focus:ring-0 rounded-lg px-4 py-2.5 text-sm transition-all outline-none text-stone-900 dark:text-stone-100 placeholder-stone-500"
                    placeholder="john@example.com"
                />
            </div>

            <div>
                <label
                    for="password"
                    class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-1.5"
                    >Пароль</label
                >
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    required
                    class="w-full bg-stone-100 dark:bg-stone-900 border border-transparent focus:border-orange-500 focus:bg-white dark:focus:bg-stone-950 focus:ring-0 rounded-lg px-4 py-2.5 text-sm transition-all outline-none text-stone-900 dark:text-stone-100 placeholder-stone-500"
                    placeholder="••••••••"
                />
            </div>

            <button
                type="submit"
                disabled={isLoading}
                class="w-full mt-2 bg-orange-500 hover:bg-orange-600 disabled:opacity-70 disabled:hover:bg-orange-500 text-white font-bold py-2.5 px-4 rounded-lg transition-colors flex justify-center items-center gap-2 shadow-sm"
            >
                {#if isLoading}
                    <svg
                        class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
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
                Зареєструватися
            </button>
        </form>

        <div
            class="mt-6 text-center text-sm text-stone-500 dark:text-stone-400"
        >
            Вже маєте акаунт?
            <a
                href="/login"
                class="text-orange-500 hover:text-orange-600 underline underline-offset-4 decoration-orange-200 dark:decoration-orange-900 transition-colors font-medium"
                >Увійти</a
            >
        </div>
    </div>
</div>
