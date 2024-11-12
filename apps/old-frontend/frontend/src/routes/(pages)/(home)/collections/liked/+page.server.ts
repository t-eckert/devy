import { API } from "$env/static/private"
import type { PageServerLoad } from "./$types"
import { type NumericRange, error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, locals }) => {
  async function fetchLikes() {
    const resp = await fetch(`${API}/collections/likes`, {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${locals.token}`
      }
    })

    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 509>, resp.statusText)
    }

    return await resp.json()
  }

  return {
    collection: await fetchLikes()
  }
}
