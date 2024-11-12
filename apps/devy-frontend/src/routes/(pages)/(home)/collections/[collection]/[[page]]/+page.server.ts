import { API } from "$env/static/private"
import type { PageServerLoad } from "./$types"
import type { Collection } from "$lib/types"
import { type NumericRange, error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, locals, params }) => {
  const { collection, page } = params

  async function fetchCollection() {
    const resp = await fetch(`${API}/collections/${collection}/${page || 1}`, {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${locals.token}`
      }
    })
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 509>, resp.statusText)
    }

    return (await resp.json()) as Collection
  }

  return {
    collection: await fetchCollection()
  }
}
