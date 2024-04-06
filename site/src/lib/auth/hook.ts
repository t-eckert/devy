import type { Handle } from "@sveltejs/kit"
import { jwtDecode } from "jwt-decode"

const handle: Handle = async ({ event, resolve }) => {
	const token = event.url.searchParams.get("token")
	if (!token) {
		return await resolve(event)
	}
	event.url.searchParams.delete("token")

	try {
		event.locals.session = jwtDecode(token)
	} catch (e) {
		console.error(e)
	}

	return await resolve(event)
}

export default handle
