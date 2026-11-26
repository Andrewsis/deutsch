// Public pages (home, verb pages) are rendered on the server so search engines see them.
// Pages that need the auth cookie or a WebSocket opt out with `ssr = false`.
export const ssr = true;
export const prerender = false;
