import { error, type NumericRange } from "@sveltejs/kit"
import { API } from "$env/static/private"
import type { PageServerLoad } from "./$types"

export const load: PageServerLoad = async ({ params, fetch }) => {
	async function fetchPost() {
		const resp = await fetch(`${API}/blogs/${params.blog}/posts/${params.slug}`, {
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
		return await resp.json()
	}

	return {
		post: await fetchPost()
	}
}
