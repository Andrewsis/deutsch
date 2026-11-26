import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load: PageLoad = async ({ params, fetch }) => {
  const verb = encodeURIComponent(params.word);


  try {
    const res = await fetch(apiUrl(`/api/word/${verb}`));

    if (!res.ok) {
      if (res.status === 404) {
        throw error(404, { message: `Дієслово "${params.word}" не знайдено.` });
      }
      throw error(res.status, { message: "Помилка бекенду" });
    }

    const word = await res.json();
    return { word };
  } catch (e) {
    console.error("problem with Rust:", e);
    throw error(500, { message: "couldnt connect to db" });
  }
};
