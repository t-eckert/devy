import type { PageServerLoad, Actions } from "./$types"
import api from "$lib/api"

export const actions: Actions = {
	setTheme: async ({ url, cookies }) => {
		const theme = url.searchParams.get("theme")

		if (theme) {
			cookies.set("colortheme", theme, {
				path: "/",
				maxAge: 60 * 60 * 24 * 365
			})
		}

		console.log(theme)
	}
}

export const load: PageServerLoad = async (event) => {
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
