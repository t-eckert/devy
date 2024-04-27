import { PageLoad } from "./$types"
import type { Feed } from "$lib/types"
import { NumericRange, error } from "@sveltejs/kit"

export const load: PageLoad = async ({ fetch, params }) => {
	const resp = await fetch(`/api/feeds/${params.feed}`)
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 509>, resp.statusText)
	}

	console.log(resp.status)

	const feed = (await resp.json()) as Feed

	return {
		feed
	}
}
