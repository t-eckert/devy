import { API } from "$env/static/private"
import type { PageServerLoad } from "./$types"
import type { Feed } from "$lib/types"
import { type NumericRange, error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, locals }) => {
	const resp = await fetch(`${API}/feeds/following`, {
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${locals.token}`
		}
	})
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 509>, resp.statusText)
	}

	const feed = (await resp.json()) as Feed

	return {
		feed
	}
}
