import type { PageLoad } from "./$types"
import { error, NumericRange } from "@sveltejs/kit"

export const load: PageLoad = async ({ params, fetch }) => {
	const blogResp = await fetch(`/api/blogs/${params.blog}`)
	if (!blogResp.ok) {
		error(blogResp.status as NumericRange<400, 599>, {
			message: blogResp.statusText
		})
	}
	const blog = await blogResp.json()

	const entriesResp = await fetch(`/api/blogs/${params.blog}/entries`)
	if (!entriesResp.ok) {
		error(blogResp.status as NumericRange<400, 599>, {
			message: blogResp.statusText
		})
	}
	const entries = await entriesResp.json()

	return {
		props: {
			blog,
			entries
		}
	}
}
