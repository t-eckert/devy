import type { Actions } from "./$types"
import { redirect } from "@sveltejs/kit"

export const actions: Actions = {
	signOut: async ({ cookies }) => {
		cookies.delete("token", { path: "/" })
		throw redirect(302, "/")
	},

	setTheme: async ({ url, cookies }) => {
		const theme = url.searchParams.get("theme")

		if (theme) {
			cookies.set("colortheme", theme, {
				path: "/",
				maxAge: 60 * 60 * 24 * 365
			})
		}
	}
}
