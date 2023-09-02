import Bookmark from "@/models/Bookmark"
import config from "@/config"

const get = {}

const upsert = async (bookmark: Bookmark) => {
	const response = await fetch(`${config.API}/bookmarks`, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(bookmark),
	})
	return await response.json()
}

const bookmark = {
	get,
	upsert,
}

export default bookmark
