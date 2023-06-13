import JSON from "@/components/elements/JSON"

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
    <div className="mx-auto mt-8 w-full max-w-6xl">
      <h1 className="text-2xl font-semibold">{post.title}</h1>
      <JSON raw={post} />
    </div>
  )
}
