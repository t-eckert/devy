import type { Handle } from "@sveltejs/kit"

export const handle: Handle = async ({ event, resolve }) => {
  {
    const err = event.url.searchParams.get("error")
    console.log("error", err)

    const token = event.url.searchParams.get("token")
    console.log("token", token)
    if (token) {
      event.cookies.set("token", token, { path: "/", maxAge: 60 * 60 * 24 * 365 })
      event.url.searchParams.delete("token")
    }
  }

  const token = event.cookies.get("token")
  event.locals.token = token

  return await resolve(event)
}
