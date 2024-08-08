import { setContext, getContext } from "svelte"
import type { Session } from "$lib/types"
import { parseSessionToken } from "$lib/auth"

class SessionState {
	signedIn = $state(false)
	session = $state<Session | null>(null)

	setToken(token: string) {
		let session
		try {
			session = parseSessionToken(token).body
			this.session = session
			this.signedIn = true
		} catch (e) {
			console.log("error", e)
		}
	}

	signOut() {
		console.log("signing out")
		this.session = null
		this.signedIn = false
	}

	isCurrentUser(userId: string): boolean {
		return this.signedIn && this.session?.userId === userId
	}

	// Returns the state of the current session.
	__debug() {
		return {
			signedIn: this.signedIn,
			session: this.session
		}
	}
}

const SESSION_KEY = Symbol("session")

export function setSessionState() {
	return setContext(SESSION_KEY, new SessionState())
}

export function getSessionState() {
	return getContext<ReturnType<typeof setSessionState>>(SESSION_KEY)
}
