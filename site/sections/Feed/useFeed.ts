import { create } from "zustand"

import { PostMetadata } from "lib/post"

export interface FeedState {
	currentFeed: string
	feeds: { name: string; posts: PostMetadata[] }[]
}

export const useFeed = create<FeedState>(() => ({
	currentFeed: "Popular",
	feeds: [{ name: "Popular", posts: [] }],
}))
