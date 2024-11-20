import type { PageServerLoad } from "./$types";
import { API } from "$env/static/private";

export const load: PageServerLoad = async ({ fetch }) => {
  async function loadProviders() {
    const res = await fetch(`${API}/auth/providers`)
    return await res.json()
  }

  return { providers: await loadProviders() }
}
