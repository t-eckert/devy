import type { PageServerLoad } from "./$types"
import type { Entry } from "$lib/types"
import { API } from "$env/static/private"
import { error, type NumericRange } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, locals }) => {
  async function fetchDrafts() {
    const resp = await fetch(`${API}/drafts`,
      {
        headers: {
          Authorization: `Bearer ${locals.token}`,
          "Content-Type": "application/json"
        }
      }
    )
    if (!resp.ok) {
      error(resp.status as NumericRange<400, 599>, {
        message: resp.statusText
      })
    }

    return (await resp.json()) as Entry[]
  }


  return {
    drafts: await fetchDrafts()
  }
}
