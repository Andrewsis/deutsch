import type { Handle, HandleFetch } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";
import { LOCALE_COOKIE, resolveLocale } from "$lib/i18n";

export const handle: Handle = async ({ event, resolve }) => {
    const lang = resolveLocale(
        event.url.searchParams.get("lang"),
        event.cookies.get(LOCALE_COOKIE),
        event.request.headers.get("accept-language"),
    );
    event.locals.lang = lang;
    return resolve(event, {
        transformPageChunk: ({ html }) => html.replace("%lang%", lang),
    });
};

/**
 * In production PUBLIC_API_URL is empty: the browser calls `/api` on the same origin and
 * nginx proxies it to the backend. The Node server is behind that nginx and cannot reach
 * `/api` on itself, so during SSR such requests go straight to API_INTERNAL_URL.
 */
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
    const internal = env.API_INTERNAL_URL?.replace(/\/+$/, "");
    const url = new URL(request.url);
    if (internal && url.origin === event.url.origin && url.pathname.startsWith("/api/")) {
        request = new Request(`${internal}${url.pathname}${url.search}`, request);
    }
    return fetch(request);
};
