import { redirect } from "@sveltejs/kit"

export function load() {
	const loggedIn = false

	if (!loggedIn) {
		throw redirect(307, "/sign-in")
	}
}
