import { setContext, getContext } from "svelte"
import type { Session } from "$lib/types"

class SessionState {
	loggedIn = $state(false)
	session = $state<Session | null>(null)

	constructor() {}
}

const SESSION_KEY = Symbol("session")

export function setSessionState() {
	return setContext(SESSION_KEY, new SessionState())
}

export function getSessionState() {
	return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY)
}
