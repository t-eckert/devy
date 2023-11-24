"use client"

import { useState } from "react"

import { Feed } from "@/models"
import PostCollection from "@/components/post-collection"

interface Props {
	feed?: Feed
}

export default function Feed({ feed }: Props) {
	const [page, setPage] = useState<number>(0)

	if (!feed) {
		throw new Error("Not found")
	}

	const loadMore = () => setPage(page + 1)

	return <PostCollection posts={feed.posts} loadMore={loadMore} />
}
