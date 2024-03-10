import type { Handle } from "@sveltejs/kit"
import { jwtDecode } from "jwt-decode"

const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get("token")
	console.log("token", token)
	if (!token) {
		return await resolve(event)
	}

	try {
		event.locals.session = jwtDecode(token)
		console.log(event.locals)
	} catch (e) {
		console.error(e)
	}

	return await resolve(event)
}

export default handle
