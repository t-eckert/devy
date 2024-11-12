import { redirect } from "@sveltejs/kit"
import { parseSessionToken } from "$lib/auth"
import type { PageServerLoad, Actions } from "./$types";
import { API } from "$env/static/private"


export const actions = {
  deleteBlog: async ({ request, fetch, locals, }) => {
    const formData = await request.formData()

    const blogSlug = formData.get("blog-slug")
    if (!blogSlug) return { status: 400 }

    const resp = await fetch(`${API}/blogs/${blogSlug}`, {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${locals.token}`,
        "Content-Type": "application/json"
      }
    })

    return {
      status: resp.status
    }
  }
} satisfies Actions


export const load: PageServerLoad = async ({ fetch, locals }) => {
  const { token } = locals

  if (!token) {
    return redirect(307, "/auth/login")
  }

  const session = parseSessionToken(token).body
  const resp = await fetch(`${API}/profiles/${session.username}/blogs`, {
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`
    }
  })

  return {
    blogs: await resp.json()
  }
}
