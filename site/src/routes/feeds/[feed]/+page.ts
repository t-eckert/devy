import { PageLoad } from "./$types"
import type { Feed } from "$lib/types"

export const load: PageLoad = async ({ fetch, params }) => {
	const resp = await fetch(`/api/feeds/${params.feed}`)
	const feed = (await resp.json()) as Feed

	return {
		feed
	}
}
