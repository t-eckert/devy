import sessionStore from "$lib/state/session-store"
import { redirect } from "@sveltejs/kit"

export function load() {
	let loggedIn = false

	sessionStore.subscribe((session) => {
		loggedIn = session.loggedIn
	})

	if (!loggedIn) {
		throw redirect(307, "/sign-in")
	}
}
