import type { Handle } from "@sveltejs/kit"
import authHandle from "$lib/auth/hook"

export const handle: Handle = async ({ event, resolve }) => {
	return authHandle({ event, resolve })
}
