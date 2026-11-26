import type { RequestHandler } from "./$types";

// Generated (not in static/) so the sitemap URL uses the real origin.
export const GET: RequestHandler = ({ url }) => {
    const body = `User-agent: *
Disallow: /api/
Disallow: /study
Disallow: /arena
Disallow: /login
Disallow: /register

Sitemap: ${url.origin}/sitemap.xml
`;
    return new Response(body, {
        headers: { "Content-Type": "text/plain; charset=utf-8", "Cache-Control": "public, max-age=86400" },
    });
};
