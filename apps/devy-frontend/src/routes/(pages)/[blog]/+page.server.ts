import type { Actions, PageServerLoad } from "./$types"
import { API } from "$env/static/private"
import { error, type NumericRange } from "@sveltejs/kit"

export const actions = {
  follow: async ({ fetch, locals, request }) => {
  }
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
