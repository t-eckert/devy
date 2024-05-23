import { writable } from "svelte/store"
import type { Handle } from "@sveltejs/kit"
import { jwtDecode } from "jwt-decode"
import type { Session, TokenPayload } from "$lib/types"

export interface SessionStore {
	loggedIn: boolean
	session?: Session
}

export const sessionStore = writable<SessionStore>({
	loggedIn: false,
	session: undefined
})

export const setSession = (session: Session) => {
	sessionStore.set({ loggedIn: true, session: session })
}

export const clearSession = () => {
	sessionStore.set({ loggedIn: false, session: undefined })
}

export const handleSession: Handle = async ({ event, resolve }) => {
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
		event.locals.session = jwtDecode<TokenPayload>(token).value
	} catch (e) {
		console.error(e)
	}

	return await resolve(event)
}
