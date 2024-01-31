import Markdown from "@/components/markdown"
import PostHeader from "@/components/post/post-header"
import Post from "@/models/Post"

import data from "./data"

interface Props {
  params: {
    blog: string
    post: string
  }
}

export default async function Post({ params }: Props) {
  const { post } = await data(params.blog, params.post)

  return (
    <main className="mx-auto my-4 flex flex-col px-3 py-4 sm:py-12 w-full max-w-6xl gap-4 sm:gap-2">
      <PostHeader post={post} />
      <Markdown content={post.content} />
    </main>
  )
}
