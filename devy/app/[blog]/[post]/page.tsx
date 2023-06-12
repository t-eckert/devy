import { getPostByBlogAndSlug } from "@/models/Post"

export default async function Post({
  params,
}: {
  params: { blog: string; slug: string }
}) {
  const post = await getPostByBlogAndSlug(params.blog, params.slug)

  if (!post) {
    return <div>Post not found</div>
  }

  return (
    <div>
      <h1>{post.slug}</h1>
      <p>in {post.blog}</p>
      <div>{post.markdown}</div>
    </div>
  )
}
