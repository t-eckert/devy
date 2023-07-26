import config from "@/config"
import Feed from "@/models/Feed"

const get = {
	new: async () => await get.byId("new"),
	popular: async () => await get.byId("popular"),
	byId: async (id: string): Promise<Option<Feed>> => {
		const res = await fetch(`${config.API}/feeds/${id}`, {
			next: { revalidate: 10 },
		})
		const feed = await res.json()
		return feed
	},
}

const feed = {
	get,
}

export default feed
