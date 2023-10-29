import Link from "@/components/link"
import Date from "@/components/date"
import Markdown from "@/components/markdown"

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
    <main className="mx-auto my-4 flex flex-col px-2 w-full max-w-2xl gap-4 sm:gap-2">
      <h1 className="text-3xl font-semibold">{post.title}</h1>

      <div className="mb-2 flex flex-row gap-2 items-baseline ">
        <div className="bg-zinc-800 text-sm flex items-center justify-center px-2 py-1 rounded-full">
          <Date date={post.createdAt} />
        </div>
        <Link
          href={`/profiles/${post.authorSlug}`}
          variant={{ underline: true }}
        >
          {post.authorName}
        </Link>
        <span>/</span>
        <Link href={`/${post.blogSlug}`} variant={{ underline: true }}>
          {post.blogName}
        </Link>
      </div>

      <Markdown content={post.content} />
    </main>
  )
}
