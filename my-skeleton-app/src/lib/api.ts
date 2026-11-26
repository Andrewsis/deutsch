import { PUBLIC_API_URL } from "$env/static/public";

// Empty PUBLIC_API_URL means the API is on the same origin (prod behind nginx).
const base = (PUBLIC_API_URL ?? "").replace(/\/+$/, "");

export function apiUrl(path: string): string {
    return `${base}${path}`;
}

export function wsUrl(path: string): string {
    if (base) {
        return base.replace(/^http/, "ws") + path;
    }
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    return `${proto}//${location.host}${path}`;
}
