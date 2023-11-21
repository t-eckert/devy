"use client"

import { useState } from "react"

import { FeedMetadata, Post } from "@/models"
import PostCollection from "@/components/post-collection"

interface Props {
	feed?: { feedMetadata: FeedMetadata; posts: Post[] }
}

export default function Feed({ feed }: Props) {
	const [page, setPage] = useState<number>(0)

	if (!feed) {
		throw new Error("Not found")
	}

	const loadMore = () => setPage(page + 1)

	return <PostCollection posts={feed.posts} loadMore={loadMore} />
}
