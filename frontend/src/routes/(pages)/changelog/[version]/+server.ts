import type { RequestHandler } from "./$types"
import { env } from "$env/dynamic/private"

export const GET: RequestHandler = async ({ url, request }) => {
	const version = url.pathname.replace("/changelog/", "")

	return await fetch(`${env.CHANGELOG}/${version}`)
}
