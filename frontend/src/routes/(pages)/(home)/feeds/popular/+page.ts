import { PUBLIC_API } from "$env/static/public"
import type { PageLoad } from "./$types"
import type { Feed } from "$lib/types"
import { type NumericRange, error } from "@sveltejs/kit"

export const load: PageLoad = async ({ fetch }) => {
  async function fetchPopular() {
    const resp = await fetch(`${PUBLIC_API}/feeds/popular`)

    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 509>, resp.statusText)
    }

    return (await resp.json()) as Feed
  }

  return {
    feed: await fetchPopular()
  }
}
