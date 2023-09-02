import Like from "@/models/Like"
import config from "@/config"

const get = {}

const upsert = async (like: Like): Promise<Like> => {
	const response = await fetch(`${config.API}/likes`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(like),
	})
	return await response.json()
}

const like = {
	get,
	upsert,
}

export default like
