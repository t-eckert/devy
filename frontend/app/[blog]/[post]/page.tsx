import Markdown from "@/components/markdown"

import PostHeader from "./post.header"

import Post from "@/models/Post"
import api from "@/lib/api"

interface Props {
  params: {
    blog: string
    post: string
  }
}

export default async function Post({ params }: Props) {
  const post = await api.get<Post>(
    `/v1/blogs/${params.blog}/posts/${params.post}`,
    600
  )

  if (!post) return <div>Post not found</div>

  return (
    <main className="mx-auto my-4 flex flex-col px-2 py-12 w-full max-w-4xl gap-4 sm:gap-2">
      <PostHeader post={post} />
      <Markdown content={post.content} />
    </main>
  )
}
