import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals, url, depends }) => {
    // Re-run when the language switcher changes the cookie or the `?lang=` parameter.
    depends("app:lang");
    url.searchParams.get("lang");
    return { lang: locals.lang };
};
