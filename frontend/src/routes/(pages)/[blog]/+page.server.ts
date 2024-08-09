import type { Actions } from "./$types"
import { API } from "$env/static/private"

export const actions = {
	delete: async ({ fetch, locals, params }) => {
		const resp = await fetch(`${API}/blogs/${params.blog}`, {
			method: "DELETE",
			headers: {
				Authorization: `Bearer ${locals.token}`,
				"Content-Type": "application/json"
			}
		})

		return {
			status: resp.status
		}
	}
} satisfies Actions
