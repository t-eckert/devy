import Link from "@/components/link"
import RelativeDate from "@/components/relative-date"
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
    <main className="mx-auto my-4 flex flex-col px-2 py-12 w-full max-w-4xl gap-4 sm:gap-2">
      <div className="pb-12 flex flex-col gap-6">
        <div className="bg-zinc-800 text-sm flex items-center justify-start rounded-full gap-4">
          <Link href={{ pathname: `/${post.blogSlug}` }} variant={{ underline: false }} className="text-neutral hover:text-neutral-1 hover:dark:text-neutral+1">
            {post.blogName}
          </Link>
          <RelativeDate date={post.createdAt} className="text-neutral" />
        </div>
        <h1 className="text-6xl font-semibold text-neutral-1 dark:text-neutral+1">
          {post.title}
        </h1>
        <div className="bg-zinc-800 text-sm flex items-center justify-start rounded-full gap-4">
          <Link
            href={{ pathname: `/profiles/${post.authorSlug}` }}
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
