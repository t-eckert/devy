import { API } from "$env/static/private"
import type { RequestEvent, RequestHandler } from "./$types"

const forwardingHeaders = new Set([
  "accept",
  "accept-encoding",
  "accept-language",
  "cache-control",
  "pragma",
  "referer",
  "user-agent",
  "x-requested-with",
  "cookie",
  "authorization",
  "content-type",
  "content-length",
  "origin"
])

// This API server function forwards requests sent to `/api` to the `devy-api` application.
// It is generally prefered to interface with `devy-api` directly through server components,
// but this implementation is useful for frontend state store interactions with the database.
export const fallback: RequestHandler = async ({ url, request, fetch, cookies }: RequestEvent) => {
  const apiPath = url.pathname.replace("/api", "")
  const { method, headers: incomingHeaders } = request

  const headers = new Headers()
  incomingHeaders.forEach((value, key) => {
    if (forwardingHeaders.has(key)) headers.set(key, value)
  })

  const token = cookies.get("token")
  if (token) {
    headers.set("Authorization", "Bearer " + token)
  }

  const body = request.body ? request.body.pipeThrough(new TransformStream()) : null;

  return await fetch(API + apiPath, {
    method,
    headers,
    body,
    // @ts-expect-error duplex is available in my Node environment.
    duplex: "half"
  })
}
