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
const STORAGE_KEY = "locale";

function isLocale(value: unknown): value is Locale {
    return LOCALES.some((l) => l.code === value);
}

function initialLocale(): Locale {
    if (typeof window === "undefined") return "uk";
    const saved = localStorage.getItem(STORAGE_KEY);
    if (isLocale(saved)) return saved;
    return navigator.language?.toLowerCase().startsWith("uk") ? "uk" : "en";
}

export const locale = writable<Locale>(initialLocale());

if (typeof window !== "undefined") {
    locale.subscribe((value) => {
        localStorage.setItem(STORAGE_KEY, value);
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

/** Non-reactive lookup for plain .ts code (load functions etc.). */
export function tr(key: MessageKey, params?: Record<string, string | number>): string {
    return translate(get(locale), key, params);
}

export type { MessageKey };
