import api from "@/lib/api"

import { Feed, FeedConfig, Post } from "@/models"

const cacheSeconds = 600

export default async function fetchFeed(
	id: string,
	page: number,
	size: number
): Promise<Feed> {
	if (page < 0) {
		throw new Error("Page cannot be negative")
	}

	const offset = page * size

	const feedConfig = await api.get<FeedConfig>(
		`/v1/feeds/${id}`,
		cacheSeconds
	)
	const posts = await api.get<Post[]>(
		`/v1/feeds/${id}/posts?offset=${offset}&limit=${size}`,
		cacheSeconds
	)

	return { feedConfig, posts }
}
