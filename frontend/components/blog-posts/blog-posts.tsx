"use client"

import Post from "@/models/Post"

interface Props {
  posts: Post[]
}

export default function BlogPosts({ posts }: Props) {
  return <div>Blog posts</div>
}
