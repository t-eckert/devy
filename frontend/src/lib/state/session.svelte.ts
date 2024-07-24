import { setContext, getContext } from "svelte"
import type { Session, TokenPayload } from "$lib/types"
import { jwtDecode } from "jwt-decode"

class SessionState {
	loggedIn = $state(false)
	session = $state<Session | null>(null)

	constructor() {}

	loadToken(token: string) {
		this.session = jwtDecode<TokenPayload<Session>>(token).body
		this.loggedIn = true
	}
}

const SESSION_KEY = Symbol("session")

export function setSessionState() {
	return setContext(SESSION_KEY, new SessionState())
}

export function getSessionState() {
	return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY)
}
