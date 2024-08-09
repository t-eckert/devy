import type { RequestHandler } from "./$types"
import { CHANGELOG } from "$env/static/private"

export const GET: RequestHandler = async ({ url, fetch }) => {
	const version = url.pathname.replace("/changelog/", "")

	return await fetch(`${CHANGELOG}/${version}`)
}
