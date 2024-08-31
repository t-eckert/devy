import type { PageLoad } from "./$types"
import { error, type NumericRange } from "@sveltejs/kit"
import { PUBLIC_API } from "$env/static/public"

export const load: PageLoad = async ({ params, fetch }) => {
  async function fetchEntry() {
    const resp = await fetch(`${PUBLIC_API}/blogs/${params.blog}/entries/${params.slug}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json"
      }
    })
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 599>, {
        message: resp.statusText
      })
    }
    return await resp.json()
  }

  async function fetchPost() {
    const resp = await fetch(`${PUBLIC_API}/blogs/${params.blog}/posts/${params.slug}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json"
      }
    })
    if (!resp.ok) {
      throw error(resp.status as NumericRange<400, 599>, {
        message: resp.statusText
      })
    }
    return await resp.json()
  }


  return {
    post: await fetchPost(),
    entry: await fetchEntry()
  }
}
