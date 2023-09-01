import Profile from "@/models/Profile"
import config from "@/config"

const get = {
	byId: async (id: string): Promise<Option<Profile>> => {
		try {
			const res = await fetch(`${config.API}/profiles/${id}`, {
				next: { revalidate: 0 },
			})

			if (!res.ok) return null

			return await res.json()
		} catch (err) {
			console.error(err)
			return null
		}
	},
}

const profileController = {
	get,
}

export default profileController
