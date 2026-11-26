import { PUBLIC_API_URL } from "$env/static/public";
import { get } from "svelte/store";
import { locale } from "$lib/i18n";

// Empty PUBLIC_API_URL means the API is on the same origin (prod behind nginx).
const base = (PUBLIC_API_URL ?? "").replace(/\/+$/, "");

export function apiUrl(path: string): string {
    return `${base}${path}`;
}

/** Appends the current translation language as `lang=` to an API path. */
export function withLang(path: string): string {
    const sep = path.includes("?") ? "&" : "?";
    return `${path}${sep}lang=${get(locale)}`;
}

export function wsUrl(path: string): string {
    if (base) {
        return base.replace(/^http/, "ws") + path;
    }
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    return `${proto}//${location.host}${path}`;
}
