import { error, redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { apiUrl, withLang } from "$lib/api";
import { translate } from "$lib/i18n";

export const load: PageLoad = async ({ params, fetch, depends, parent, url }) => {
  depends("app:lang");
  // Runs on the server too: take the language from layout data, not from the shared store.
  const { lang } = await parent();
  const verb = encodeURIComponent(params.word);

  let res: Response;
  let sentencesRes: Response | null;
  try {
    [res, sentencesRes] = await Promise.all([
      fetch(apiUrl(withLang(`/api/word/${verb}`, lang))),
      fetch(apiUrl(withLang(`/api/sentences?verb=${verb}&limit=10`, lang))).catch(() => null),
    ]);
  } catch (e) {
    console.error("problem with Rust:", e);
    throw error(500, { message: translate(lang, "common.noConnection") });
  }

  if (!res.ok) {
    if (res.status === 404) {
      // Maybe an inflected form ("wartete"): go to its verb.
      const lemmaRes = await fetch(apiUrl(`/api/lemma?form=${verb}`)).catch(() => null);
      const lemmas: string[] = lemmaRes?.ok ? await lemmaRes.json() : [];
      if (lemmas.length > 0 && lemmas[0] !== params.word) {
        // Permanent, so search engines index the verb page instead of the form.
        throw redirect(301, `/${encodeURIComponent(lemmas[0])}${url.search}`);
      }
      throw error(404, { message: translate(lang, "word.notFound", { word: params.word }) });
    }
    throw error(res.status, { message: translate(lang, "word.backendError") });
  }

  const word = await res.json();
  const sentences = sentencesRes?.ok ? await sentencesRes.json() : [];
  return { word, sentences, lang };
};
