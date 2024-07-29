import type { Actions } from "./$types"

export const actions = {
	default: async (event) => {
		event.cookies.delete("token", {
			path: "/"
		})
	}
} satisfies Actions
