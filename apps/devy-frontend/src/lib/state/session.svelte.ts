import { getContext, setContext } from "svelte"
import { sessionOrNull } from "$lib/auth"
import type { Session } from "$lib/types"

class CurrentSession {
	private token = $state<string>()
	private session: Session | null = $derived(sessionOrNull(this.token))

	isAuthenticated: boolean = $derived(this.session !== null)

	userId = $derived(this.session?.userId)
	profileId = $derived(this.session?.profileId)
	username = $derived(this.session?.username)
	role = $derived(this.session?.role)
	status = $derived(this.session?.status)
	displayName = $derived(this.session?.displayName)
	avatarUrl = $derived(this.session?.avatarUrl)

	constructor(token?: string) {
		if (token === undefined) {
			return
		}

		this.token = token
	}

	setToken(token: string) {
		this.token = token
	}

	clearToken() {
		this.token = undefined
	}
}

const SESSION_KEY = Symbol("current-session")

export function setSession(token?: string) {
	return setContext(SESSION_KEY, new CurrentSession(token))
}

export function getSession() {
	return getContext<ReturnType<typeof setSession>>(SESSION_KEY)
}
