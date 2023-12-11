"use client"

import Likes from "@/components/likes"
import Link from "@/components/link"
import RelativeDate from "@/components/relative-date"

import { Post } from "@/models"

interface Props {
  post: Post
}

export default function PostHeader({ post }: Props) {
  return (
    <div className="pb-6 sm:pb-12 flex flex-col gap-6">
      <div className="mx-auto w-full max-w-2xl text-sm flex flex-row justify-between sm:justify-start sm:gap-4">
        <Link
          href={{ pathname: `/${post.blogSlug}` }}
          variant={{ underline: false }}
          className="text-neutral hover:text-neutral-1 hover:dark:text-neutral+1"
        >
          {post.blogName}
        </Link>
        <RelativeDate
          date={post.createdAt}
          className="text-neutral select-none"
        />
      </div>

      <h1 className="text-4xl sm:text-6xl font-semibold text-neutral-1 dark:text-neutral+1">
        {post.title}
      </h1>

      <div className="mx-auto w-full max-w-2xl text-sm flex flex-row items-baseline justify-between sm:flex-col sm:gap-4">
        <div className="">
          <Link
            href={{ pathname: `/profiles/${post.authorSlug}` }}
            variant={{ underline: false }}
          >
            {post.authorName}
          </Link>
        </div>
        <div className="w-12 flex flex-col items-start gap-1">
          <Likes postId={post.id} initialCount={post.likes} />
        </div>
      </div>
    </div>
  )
}
