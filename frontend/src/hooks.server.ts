import { jwtDecode } from "jwt-decode"
import type { Session } from "$lib/types"
import type { Handle } from "@sveltejs/kit"

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get("token")
	event.locals.token = token

	return await resolve(event)
}
