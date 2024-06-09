import { writable } from "svelte/store"
import type { Session } from "$lib/types"

export interface SessionStore {
	loggedIn: boolean
	session?: Session
}

const sessionStore = writable<SessionStore>({
	loggedIn: false,
	session: undefined
})

export default sessionStore

export const setSession = (session: Session) => {
	sessionStore.set({ loggedIn: true, session: session })
}

export const clearSession = () => {
	sessionStore.set({ loggedIn: false, session: undefined })
}
