import type { Actions, PageServerLoad } from "./$types"
import { API } from "$env/static/private"
import { parseSessionToken } from "$lib/auth"
import { error, type NumericRange } from "@sveltejs/kit"

export const actions = {
  follow: async ({ fetch, locals, request }) => {
    if (!locals.token) return

    const formData = await request.formData()
    const action = formData.get("action")
    const blogId = formData.get("blogId")
    const profileId = parseSessionToken(locals.token).body.profileId

    if (action === "follow") {
      await fetch(`${API}/follows`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${locals.token}`,
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          profileId,
          blogId
        })
      })
    } else if (action === "unfollow") {
      await fetch(`${API}/follows/${profileId}/${blogId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${locals.token}`,
          "Content-Type": "application/json"
        }
      })
    }
  },
} satisfies Actions

export const load: PageServerLoad = async ({ params, fetch, locals }) => {
  async function fetchBlog() {
    const blogResp = await fetch(`${API}/blogs/${params.blog}`)
    if (!blogResp.ok) {
      error(blogResp.status as NumericRange<400, 599>, {
        message: blogResp.statusText
      })
    }
    return await blogResp.json()
  }

  async function fetchEntries() {
    const entriesResp = await fetch(`${API}/blogs/${params.blog}/entries`)
    if (!entriesResp.ok) {
      error(entriesResp.status as NumericRange<400, 599>, {
        message: entriesResp.statusText
      })
    }
    return await entriesResp.json()
  }

  async function fetchIsUserFollowing(blogId: string) {
    if (locals.token === undefined) return false

    const profileId = parseSessionToken(locals.token).body.profileId

    const resp = await fetch(`${API}/follows/${profileId}`)
    if (resp.status !== 200) return false

    const follows: { profileId: string; blogId: string }[] = await resp.json()
    return follows.some((f) => f.profileId === profileId && f.blogId === blogId)
  }

  const blog = await fetchBlog()

  return {
    blog,
    entries: await fetchEntries(),
    isUserFollowing: await fetchIsUserFollowing(blog.id)
  }
}
