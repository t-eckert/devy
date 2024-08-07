import type { RequestHandler } from "./$types"
import { env } from "$env/dynamic/private"

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

export const fallback: RequestHandler = async ({ url, request }) => {
	const apiPath = url.pathname.replace("/api", "")
	const { method, headers: incomingHeaders } = request

	const headers = new Headers()
	incomingHeaders.forEach((value, key) => {
		if (forwardingHeaders.has(key)) headers.set(key, value)
	})

	return await fetch(env.API + apiPath, {
		method,
		headers
	})
}
