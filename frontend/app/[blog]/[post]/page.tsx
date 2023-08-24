import Json from "@/components/debugging/Json"
import { Link } from "@/components/elements"

import postController from "@/controllers/post"
import Post from "@/models/Post"

interface Props {
  params: {
    blog: string
    post: string
  }
}

export default async function Post({ params }: Props) {
  const post: Post = await postController.get.byBlogSlugAndPostSlug(
    params.blog,
    params.post
  )

  return (
    <main className="mx-auto my-4 flex flex-col px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Json data={post} />
      <h1>{post.title}</h1>
      <section className="flex flex-row gap-2">
        <span>
          <Link
            href={`/profiles/${post.authorSlug}`}
            variant={{ underline: true }}
          >
            {post.authorName}
          </Link>
        </span>
        <span>
          <Link href={`/${post.blogSlug}`} variant={{ underline: true }}>
            {post.blogName}
          </Link>
        </span>
      </section>
      <div>{post.content}</div>
    </main>
  )
}
