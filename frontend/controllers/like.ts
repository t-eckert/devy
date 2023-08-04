import Like from "@/models/Like"
import config from "@/config"

const upsert = async (like: Like): Promise<Like> => {
	const response = await fetch(`${config.HOST}/api/likes`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(like),
	})
	return await response.json()
}

const like = {
	upsert,
}

export default like
