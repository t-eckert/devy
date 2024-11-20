import type { PageServerLoad } from "./$types";
import { API } from "$env/static/private";

export const load: PageServerLoad = async (event) => {
  console.log(event)

  return {}
}
