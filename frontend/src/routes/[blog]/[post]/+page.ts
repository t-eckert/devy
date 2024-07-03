import type { PageLoad } from "./$types"
import { error, NumericRange } from "@sveltejs/kit"

export const load: PageLoad = async ({ params, fetch }) => {
	const resp = await fetch(`/api/blogs/${params.blog}/entries/${params.post}`)
	if (!resp.ok) {
		return error(resp.status as NumericRange<400, 599>, {
			message: resp.statusText
		})
	}
	const entry = await resp.json()

	return {
		entry
	}
}
