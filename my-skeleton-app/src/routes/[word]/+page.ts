import { error, redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { apiUrl, withLang } from "$lib/api";
import { tr, locale } from "$lib/i18n";
import { get } from "svelte/store";

export const load: PageLoad = async ({ params, fetch, depends }) => {
  depends("app:lang");
  const verb = encodeURIComponent(params.word);

  let res: Response;
  let sentencesRes: Response | null;
  try {
    [res, sentencesRes] = await Promise.all([
      fetch(apiUrl(withLang(`/api/word/${verb}`))),
      fetch(apiUrl(withLang(`/api/sentences?verb=${verb}&limit=10`))).catch(() => null),
    ]);
  } catch (e) {
    console.error("problem with Rust:", e);
    throw error(500, { message: tr("common.noConnection") });
  }

  if (!res.ok) {
    if (res.status === 404) {
      // Maybe an inflected form ("wartete"): go to its verb.
      const lemmaRes = await fetch(apiUrl(`/api/lemma?form=${verb}`)).catch(() => null);
      const lemmas: string[] = lemmaRes?.ok ? await lemmaRes.json() : [];
      if (lemmas.length > 0 && lemmas[0] !== params.word) {
        throw redirect(307, `/${encodeURIComponent(lemmas[0])}`);
      }
      throw error(404, { message: tr("word.notFound", { word: params.word }) });
    }
    throw error(res.status, { message: tr("word.backendError") });
  }

  const word = await res.json();
  const sentences = sentencesRes?.ok ? await sentencesRes.json() : [];
  return { word, sentences, lang: get(locale) };
};
