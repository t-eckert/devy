import type { PageLoad } from "./$types"
import { error, type NumericRange } from "@sveltejs/kit"
import { PUBLIC_API } from "$env/static/public"

export const load: PageLoad = async ({ params, fetch }) => {
	const resp = await fetch(`${PUBLIC_API}/blogs/${params.blog}/entries/${params.post}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json"
		}
	})
	if (!resp.ok) {
		throw error(resp.status as NumericRange<400, 599>, {
			message: resp.statusText
		})
	}
	const entry = await resp.json()

	return {
		entry
	}
}
