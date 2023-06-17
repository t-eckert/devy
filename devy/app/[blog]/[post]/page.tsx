import JSON from "@/components/elements/JSON"
import Article from "@/components/markdown/Article"
import { Byline } from "@/components/models/Post"

import { postGetter } from "@/models/Post"

interface Props {
  params: { blog: string; slug: string }
}

export default async function Post({ params: { blog, slug } }: Props) {
  const post = await postGetter.byBlogAndSlug(blog, slug)

  if (!post) {
    return <div>Post not found</div>
  }

  return (
    <div className="mx-auto mt-8 w-full max-w-2xl">
      <h1 className="text-4xl font-semibold">{post.title}</h1>
      <Byline {...post} />
      <Article markdown={post.markdown} />
    </div>
  )
}
