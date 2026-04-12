import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { browser } from "$app/environment";

export const load: PageLoad = async ({ params, fetch }) => {
  const verb = encodeURIComponent(params.word);

  const targetIp = browser ? "192.168.1.71" : "127.0.0.1";

  try {
    const res = await fetch(`http://${targetIp}:8080/api/word/${verb}`);

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
