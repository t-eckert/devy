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

		return await res.json()
	},
}

const upsert = async (_: Feed): Promise<Option<Feed>> => {
	return null
}

const feed: Controller<Feed> = {
	get,
	upsert,
}

export default feed
