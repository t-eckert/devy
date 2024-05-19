import type { Handle } from "@sveltejs/kit"
import { jwtDecode } from "jwt-decode"
import type { TokenPayload } from "$lib/types"

const handle: Handle = async ({ event, resolve }) => {
	{
		const token = event.url.searchParams.get("token")
		if (token !== null) {
			event.cookies.set("token", token, {
				path: "/",
				maxAge: 60 * 60 * 24 * 365
			})
			event.url.searchParams.delete("token")
		}
	}

	const token = event.cookies.get("token")
	if (token === undefined) {
		return await resolve(event)
	}

	try {
		const body = jwtDecode<TokenPayload>(token)
		event.locals.session = body.value
	} catch (e) {
		console.error(e)
	}

	return await resolve(event)
}

export default handle
