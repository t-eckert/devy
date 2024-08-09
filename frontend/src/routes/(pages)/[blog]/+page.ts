import type { PageLoad } from "./$types"
import { error, type NumericRange } from "@sveltejs/kit"
import { PUBLIC_API } from "$env/static/public"

export const load: PageLoad = async ({ params, fetch }) => {
	const blogResp = await fetch(`${PUBLIC_API}/blogs/${params.blog}`)
	if (!blogResp.ok) {
		error(blogResp.status as NumericRange<400, 599>, {
			message: blogResp.statusText
		})
	}
	const blog = await blogResp.json()

	const entriesResp = await fetch(`${PUBLIC_API}/blogs/${params.blog}/entries`)
	if (!entriesResp.ok) {
		error(blogResp.status as NumericRange<400, 599>, {
			message: blogResp.statusText
		})
	}
	const entries = await entriesResp.json()

	return {
		blog,
		entries
	}
}
