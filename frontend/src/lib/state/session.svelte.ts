import { setContext, getContext } from "svelte"
import type { Session, TokenPayload } from "$lib/types"
import { jwtDecode } from "jwt-decode"

class SessionState {
	signedIn = $state(false)
	session = $state<Session | null>(null)

	constructor() {}

	loadToken(token: string) {
		let session
		try {
			session = jwtDecode<TokenPayload<Session>>(token).body
			this.session = session
			this.signedIn = true
		} catch (e) {
			console.log("error", e)
		}
	}

	signOut() {
		document.cookie = "token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
		this.session = null
		this.signedIn = false
	}
}

const SESSION_KEY = Symbol("session")

export function setSessionState() {
	return setContext(SESSION_KEY, new SessionState())
}

export function getSessionState() {
	return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY)
}
