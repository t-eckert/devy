import type { PageServerLoad, Actions } from "./$types"
import { redirect } from "@sveltejs/kit"
import type { Feed } from "$lib/types"
import api from "$lib/api"

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

export const load: PageServerLoad = async () => {
	const feeds: Feed[] = await Promise.all(
		["recent"].map(async (feed) => await api.get("/feeds/" + feed))
	)

	return {
		feeds
	}
}
