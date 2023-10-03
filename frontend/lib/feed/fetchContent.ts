import type Content from "./Content"
import type { Feed, Post } from "@/models"
import api from "@/lib/api"

const cacheSeconds = 600

export default async function fetchContent(
	id: string,
	page: number,
	pageSize: number
): Promise<Option<Content>> {
	if (page < 0) return null

	const offset = pageSize * page

	const url = `/feeds/${id}/posts?offset=${offset}&limit=${pageSize}`

	const feed = await api.get<Feed>(`/feeds/${id}`, cacheSeconds)
	const posts = await api.get<Post[]>(url, cacheSeconds)

	if (!feed) return null

	return {
		feed,
		posts: posts || [],
	}
}
