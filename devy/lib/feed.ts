import type Feed from "@/models/Feed"

export const feedPopular = {
	id: 1,
	slug: "popular",
	name: "Popular",
	publishOrder: "desc",
	filterTags: [],
	lastCachedAt: new Date(),
	posts: [],
}

export const feedNew = {
	id: 2,
	slug: "new",
	name: "New",
	publishOrder: "desc",
	filterTags: [],
	lastCachedAt: new Date(),
	posts: [],
}
