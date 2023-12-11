"use client"

import { Feed } from "@/models"
import PostCollection from "@/components/post-collection"

interface Props {
	feed?: Feed
}

export default function Feed({ feed }: Props) {
	return <PostCollection posts={feed?.posts || []} />
}
