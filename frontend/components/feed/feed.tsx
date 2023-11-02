"use client"

import { useState } from "react"

import { FeedMetadata, Post } from "@/models"
import PostCollection from "@/components/post-collection"

const pageSize = 15

interface Props {
	feed?: { feedMetadata: FeedMetadata; posts: Post[] }
}

export default function Feed({ feed }: Props) {
	const [page, setPage] = useState<number>(0)

	if (!feed) {
		throw new Error("Not found")
	}

	const next = {
		exists: false,
		onNavigate: () => setPage(page + 1),
	}

	const prev = {
		exists: page > 0,
		onNavigate: () => setPage(page - 1),
	}

	return <PostCollection posts={feed.posts} next={next} prev={prev} />
}
