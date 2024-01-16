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
    <div className="mx-auto w-full max-w-3xl pb-6 sm:pb-12 flex flex-col gap-12">
      <div className="mx-auto w-full text-sm flex flex-row justify-between sm:gap-4">
        <Link
          href={{ pathname: `/${post.blogSlug}` }}
          variant={{ underline: false, styled: false }}
          className="text-neutral font-medium hover:text-neutral-1 hover:dark:text-neutral+1"
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

      <div className="mx-auto w-full text-sm flex flex-row justify-between sm:gap-4">
        <Link
          href={{ pathname: `/profiles/${post.authorUsername}` }}
          variant={{ underline: false, styled: false }}
          className="text-neutral font-medium hover:text-neutral-1 hover:dark:text-neutral+1"
        >
          {post.authorName}
        </Link>

        <Likes postId={post.id} title={post.title} initialCount={post.likes} />
      </div>
    </div>
  )
}
