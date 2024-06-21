import { PageServerLoad } from "./$types"
import type { Feed } from "$lib/types"
import { NumericRange, error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ fetch, params }) => {
	const resp = await fetch(`/api/feeds/${params.feed}`)
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 509>, resp.statusText)
	}

	const feed = (await resp.json()) as Feed

	return {
		feed
	}
}
