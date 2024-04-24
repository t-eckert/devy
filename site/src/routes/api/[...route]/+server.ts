import config from "$lib/config"
import { redirect } from "@sveltejs/kit"
import type { RequestHandler } from "./$types"

export const GET: RequestHandler = async ({ url }) => {
	const apiPath = url.pathname.replace("/api", "")
	throw redirect(302, config.api + apiPath)
}

export const POST: RequestHandler = async ({ url }) => {
	const apiPath = url.pathname.replace("/api", "")
	throw redirect(302, config.api + apiPath)
}

export const PUT: RequestHandler = async ({ url }) => {
	const apiPath = url.pathname.replace("/api", "")
	throw redirect(302, config.api + apiPath)
}

export const PATCH: RequestHandler = async ({ url }) => {
	const apiPath = url.pathname.replace("/api", "")
	throw redirect(302, config.api + apiPath)
}

export const DELETE: RequestHandler = async ({ url }) => {
	const apiPath = url.pathname.replace("/api", "")
	throw redirect(302, config.api + apiPath)
}
