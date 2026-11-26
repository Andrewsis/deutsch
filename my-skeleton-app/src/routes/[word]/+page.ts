import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { apiUrl, withLang } from "$lib/api";
import { tr } from "$lib/i18n";

export const load: PageLoad = async ({ params, fetch, depends }) => {
  depends("app:lang");
  const verb = encodeURIComponent(params.word);

  let res: Response;
  try {
    res = await fetch(apiUrl(withLang(`/api/word/${verb}`)));
  } catch (e) {
    console.error("problem with Rust:", e);
    throw error(500, { message: tr("common.noConnection") });
  }

  if (!res.ok) {
    if (res.status === 404) {
      throw error(404, { message: tr("word.notFound", { word: params.word }) });
    }
    throw error(res.status, { message: tr("word.backendError") });
  }

  const word = await res.json();
  return { word };
};
