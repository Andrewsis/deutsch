import type { MessageKey } from "$lib/i18n";

type Translate = (key: MessageKey, params?: Record<string, string | number>) => string;

type VerbForms = {
    lemma: string;
    aux?: string | null;
    prs3sg?: string | null;
    prt3sg?: string | null;
    pa2?: string | null;
    translation?: string | null;
};

const MAX_TRANSLATION = 60;

/** "hat" / "ist" / "hat/ist" for the Perfekt, from `verben_wide.aux`. */
function perfectAux(aux: string | null | undefined): string {
    const a = (aux ?? "").toLowerCase();
    const sein = a.includes("sein");
    const haben = a.includes("haben") || !sein;
    return sein && haben ? "hat/ist" : sein ? "ist" : "hat";
}

/** Title and meta description for a verb page, e.g. "er geht, ging, ist gegangen". */
export function verbSeo(w: VerbForms, t: Translate): { title: string; description: string } {
    const forms = [
        w.prs3sg && `er ${w.prs3sg}`,
        w.prt3sg && `er ${w.prt3sg}`,
        w.pa2 && `er ${perfectAux(w.aux)} ${w.pa2}`,
    ]
        .filter(Boolean)
        .join(", ");

    let translation = (w.translation ?? "").trim();
    if (translation.length > MAX_TRANSLATION) {
        const cut = translation.slice(0, MAX_TRANSLATION);
        translation = `${cut.slice(0, cut.lastIndexOf(",") > 0 ? cut.lastIndexOf(",") : MAX_TRANSLATION)}…`;
    }

    return {
        title: t("seo.wordTitle", { lemma: w.lemma }),
        description: t("seo.wordDescription", {
            lemma: w.lemma,
            translation: translation ? ` (${translation})` : "",
            forms,
        }),
    };
}
