import { derived, get, writable } from "svelte/store";
import uk, { type MessageKey } from "./uk";
import en from "./en";

/** Interface + translation languages. Codes match `verb_translations.lang` on the backend. */
export const LOCALES = [
    { code: "uk", label: "UA" },
    { code: "en", label: "EN" },
] as const;
export type Locale = (typeof LOCALES)[number]["code"];

const dictionaries: Record<Locale, Record<MessageKey, string>> = { uk, en };
export const DEFAULT_LOCALE: Locale = "uk";
/** Cookie that tells the SSR server which language to render. */
export const LOCALE_COOKIE = "locale";
const STORAGE_KEY = "locale";

export function isLocale(value: unknown): value is Locale {
    return LOCALES.some((l) => l.code === value);
}

/**
 * Language for a request: `?lang=` (shareable/indexable URLs) > cookie (user choice) >
 * Accept-Language > default. Crawlers send neither cookie nor language, so they get
 * the default language at the URL without `?lang=`.
 */
export function resolveLocale(
    query: string | null,
    cookie: string | undefined,
    acceptLanguage: string | null,
): Locale {
    if (isLocale(query)) return query;
    if (isLocale(cookie)) return cookie;
    const first = acceptLanguage?.split(",")[0]?.trim().toLowerCase();
    if (first) return first.startsWith("uk") ? "uk" : "en";
    return DEFAULT_LOCALE;
}

function initialLocale(): Locale {
    if (typeof window === "undefined") return DEFAULT_LOCALE;
    // The server rendered the page in this language (see hooks.server.ts); start with it
    // so hydration matches. The root layout then keeps the store in sync with its data.
    const rendered = document.documentElement.lang;
    if (isLocale(rendered)) return rendered;
    const saved = localStorage.getItem(STORAGE_KEY);
    if (isLocale(saved)) return saved;
    return DEFAULT_LOCALE;
}

/**
 * Current UI language. On the server this store is shared between requests, so it is only
 * set synchronously during rendering (root layout); load functions must use the `lang`
 * from layout data instead of reading it.
 */
export const locale = writable<Locale>(initialLocale());

if (typeof window !== "undefined") {
    locale.subscribe((value) => {
        localStorage.setItem(STORAGE_KEY, value);
        document.cookie = `${LOCALE_COOKIE}=${value}; path=/; max-age=31536000; samesite=lax`;
        document.documentElement.lang = value;
    });
}

export function translate(
    lang: Locale,
    key: MessageKey,
    params?: Record<string, string | number>,
): string {
    let text = dictionaries[lang][key] ?? uk[key] ?? key;
    if (params) {
        for (const [name, value] of Object.entries(params)) {
            text = text.replaceAll(`{${name}}`, String(value));
        }
    }
    return text;
}

/** `$t("key", { param })` in components. */
export const t = derived(
    locale,
    (lang) => (key: MessageKey, params?: Record<string, string | number>) =>
        translate(lang, key, params),
);

/** Non-reactive lookup for plain browser-side .ts code. In load functions use `translate(lang, …)`. */
export function tr(key: MessageKey, params?: Record<string, string | number>): string {
    return translate(get(locale), key, params);
}

export type { MessageKey };
