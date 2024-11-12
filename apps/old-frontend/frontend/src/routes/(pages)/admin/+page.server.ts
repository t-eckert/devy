import { API } from "$env/static/private"
import { error } from "@sveltejs/kit"
import type { PageServerLoad } from "./$types"
import { parseSessionToken } from "$lib/auth"
import { redirect } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ locals, fetch }) => {
  if (!locals.token) {
    return redirect(307, "/")
  }

  const session = parseSessionToken(locals.token).body

  if (session.role !== "admin") {
    return redirect(307, "/")
  }

  async function fetchUsers() {
    const resp = await fetch(API + "/users", {
      headers: {
        "Content-Type": "application/json",
        "Authorization": "Bearer " + locals.token
      }
    })

    if (!resp.ok) {
      error(resp.status, resp.statusText)
    }

    const users = await resp.json()
    return users
  }

  return {
    users: await fetchUsers()
  }
}
