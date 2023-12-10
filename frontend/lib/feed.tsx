import api from "@/lib/api"

import { Feed, FeedConfig, Post } from "@/models"

const cacheSeconds = 600

export default async function fetchFeed(
	id: string,
): Promise<Feed> {
	const feedConfig = await api.get<FeedConfig>(
		`/v1/feeds/${id}/config`,
		cacheSeconds
	)
	const posts = await api.get<Post[]>(
		`/v1/feeds/${id}/posts`,
		cacheSeconds
	)

	return { feedConfig, posts }
}
