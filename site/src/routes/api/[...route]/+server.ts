import config from "$lib/config"
import type { RequestHandler } from "./$types"

export const fallback: RequestHandler = async ({ url, request }) => {
	const apiPath = url.pathname.replace("/api", "")
	return await fetch(config.api + apiPath, request)
}
