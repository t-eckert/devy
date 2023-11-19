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
    <main className="mx-auto my-4 flex flex-col px-2 py-12 w-full max-w-6xl gap-4 sm:gap-2">
      <div className="flex flex-col gap-4">
        <div className="bg-zinc-800 text-sm flex items-center justify-start  py-1 rounded-full gap-4">
          <Link href={`/${post.blogSlug}`} variant={{ underline: false }}>
            {post.blogName}
          </Link>
          <Date date={post.createdAt} />
        </div>
        <h1 className="mb-4 text-6xl font-semibold text-neutral-1 dark:text-neutral+1">
          {post.title}
        </h1>
        <div className="bg-zinc-800 text-sm flex items-center justify-start py-1 rounded-full gap-4">
          <Link
            href={`/profiles/${post.authorSlug}`}
            variant={{ underline: false }}
          >
            {post.authorName}
          </Link>
        </div>
      </div>

      <Markdown content={post.content} />
    </main>
  )
}
