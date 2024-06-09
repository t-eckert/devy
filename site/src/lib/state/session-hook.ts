import type { Handle } from "@sveltejs/kit"
import { jwtDecode } from "jwt-decode"
import type { TokenPayload } from "$lib/types"

const sessionHookHandler: Handle = async ({ event, resolve }) => {
	// Accept the token from the URL query parameter and set it as a cookie
	const token = event.url.searchParams.get("token")
	if (token === null) {
		return await resolve(event)
	}

	// Move the token from the url to cookies.
	event.cookies.set("token", token, {
		path: "/",
		maxAge: 60 * 60 * 24 * 365
	})
	event.url.searchParams.delete("token")

	// Parse the token and retrieve the session data
	try {
		const tokenData = jwtDecode<TokenPayload>(token, {})
		console.log(tokenData)
		event.locals.session = tokenData.body
	} catch (e) {
		console.error(e)
	}

	return await resolve(event)
}

export default sessionHookHandler
