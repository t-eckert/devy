import type { PageServerLoad, Actions } from "./$types"
import { redirect } from "@sveltejs/kit"
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
	const feeds = await Promise.all(
		["new", "popular"].map(async (feed) => {
			const posts = await api.get("/v1/feeds/" + feed + "/posts")
			const config = await api.get("/v1/feeds/" + feed + "/config")

			return {
				feed,
				posts,
				config
			}
		})
	)

	return {
		props: {
			feeds
		}
	}
}
