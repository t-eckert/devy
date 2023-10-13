"use client"

import { HeartIcon, HeartFilledIcon } from "@radix-ui/react-icons"

import Post from "@/models/Post"
import { Button, Link } from "@/components/elements"

export default function Preview(post: Post) {
	const likedByUser = false

	return (
		<section className="flex flex-row gap-2 items-start">
			<div className="flex flex-col items-start gap-1">
				<Button className="flex flex-row gap-1 items-center w-14 px-2 py-0.5 rounded-md">
					{likedByUser ? (
						<HeartFilledIcon className="text-zinc-300 h-3 aspect-square" />
					) : (
						<HeartIcon className="text-zinc-300 h-3 aspect-square" />
					)}
					<span className="text-sm">{post.likes}</span>
				</Button>
			</div>

			<div className="flex flex-row items-start gap-1">
				<div className="flex flex-col gap-1">
					<Link
						href={`${post.blogSlug}/${post.slug}`}
						className="font-medium text-zinc-50"
					>
						{post.title}
					</Link>

					<div className="mb-2 flex flex-row gap-2 items-baseline text-sm">
						<Link href={`/profiles/${post.authorSlug}`}>
							{post.authorName}
						</Link>
						<Link href={`/${post.blogSlug}`}>{post.blogName}</Link>
					</div>
				</div>
			</div>
		</section>
	)
}
