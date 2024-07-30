import type { Actions } from "./$types"

export const actions = {
	delete: async ({ fetch, locals, params }) => {
		const resp = await fetch(`/api/blogs/${params.blog}`, {
			method: "DELETE",
			headers: {
				Authorization: `Bearer ${locals.token}`
			}
		})

		return {
			status: resp.status
		}
	}
} satisfies Actions
