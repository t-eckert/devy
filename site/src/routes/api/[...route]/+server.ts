import config from "$lib/config"
import { redirect } from "@sveltejs/kit"
import type { RequestHandler } from "./$types"

const forwardingHandler: RequestHandler = async ({ url, request }) => {
	const apiPath = url.pathname.replace("/api", "")
	return await fetch(config.api + apiPath, request)
}

export const GET = forwardingHandler
export const POST = forwardingHandler
export const PUT = forwardingHandler
export const PATCH = forwardingHandler
export const DELETE = forwardingHandler
