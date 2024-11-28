import type { LayoutServerLoad } from "./$types"
import { parseSessionToken } from "$lib/auth"
import { redirect } from "@sveltejs/kit"

export const load: LayoutServerLoad = async ({ locals }) => {
	if (!locals.token) {
		return redirect(307, "/")
	}

	const session = parseSessionToken(locals.token).body

	if (session.role !== "admin") {
		return redirect(307, "/")
	}
}
