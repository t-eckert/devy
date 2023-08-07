import Feed from "@/models/Feed"
import config from "@/config"

const get = {
	new: async () => await get.byId("new"),
	popular: async () => await get.byId("popular"),
	byId: async (id: string): Promise<Option<Feed>> => {
		const res = await fetch(`${config.API}/feeds/${id}`, {
			next: { revalidate: 10 },
		})

		if (!res.ok) return null

		const feed = await res.json()
		return feed
	},
}

const feed = {
	get,
}

export default feed
