import { API } from "$env/static/private"
import type { PageServerLoad } from "./$types"
import type { Feed } from "$lib/types"
import { type NumericRange, error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, locals, params, depends }) => {
  const { feed, page } = params
  depends("all")

  async function fetchFeed() {
    console.log("fetchFeed")
    const resp = await fetch(`${API}/feeds/${feed}/${page || 1}`, {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${locals.token}`
      }
    })
    console.log(resp)
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 509>, resp.statusText)
    }

    return (await resp.json()) as Feed
  }

  const feedData = await fetchFeed()

  return {
    feed: feedData
  }
}
