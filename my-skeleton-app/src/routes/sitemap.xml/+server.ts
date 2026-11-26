import { error } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { apiUrl } from "$lib/api";
import { DEFAULT_LOCALE, LOCALES } from "$lib/i18n";

// Google's limit per sitemap file; each page is listed once per language.
const MAX_URLS = 50_000;

function escapeXml(s: string): string {
    return s
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;");
}

/** One <url> per language version, each listing all versions as hreflang alternates. */
function entries(base: string): string {
    const href = (code: string) =>
        escapeXml(code === DEFAULT_LOCALE ? base : `${base}?lang=${code}`);
    const alternates = [
        ...LOCALES.map((l) => `<xhtml:link rel="alternate" hreflang="${l.code}" href="${href(l.code)}"/>`),
        `<xhtml:link rel="alternate" hreflang="x-default" href="${href(DEFAULT_LOCALE)}"/>`,
    ].join("");
    return LOCALES.map((l) => `<url><loc>${href(l.code)}</loc>${alternates}</url>`).join("\n");
}

export const GET: RequestHandler = async ({ fetch, url }) => {
    const res = await fetch(apiUrl("/api/lemmas")).catch(() => null);
    if (!res?.ok) throw error(502, "Could not load verb list");
    const lemmas: string[] = await res.json();

    const pages = ["/", ...lemmas.map((l) => `/${encodeURIComponent(l)}`)];
    const maxPages = Math.floor(MAX_URLS / LOCALES.length);
    if (pages.length > maxPages) {
        console.warn(`sitemap: ${pages.length} pages, only the first ${maxPages} fit`);
    }

    const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${pages
    .slice(0, maxPages)
    .map((p) => entries(`${url.origin}${p}`))
    .join("\n")}
</urlset>
`;
    return new Response(body, {
        headers: {
            "Content-Type": "application/xml; charset=utf-8",
            "Cache-Control": "public, max-age=3600",
        },
    });
};
