"use client"

import Post from "@/models/Post"
import like from "@/controllers/like"
import { Button, Link } from "@/components/elements"
import { Bookmark, Heart } from "lucide-react"

export default function Preview(post: Post) {
	return (
		<div className="px-2 flex flex-row items-start gap-1">
			<div className="px-0.5 py-1 flex flex-col justify-end">
				<Button
					onClick={() => {
						like.upsert({
							postId: post.id,
							profileId: 1,
						})
					}}
				>
					<div className="flex flex-row justify-end items-center gap-1">
						<span className="text-sm">{post.likes}</span>
						<Heart className="w-3.5 h-3.5 text-zinc-200" />
					</div>
				</Button>
				<Button>
					<div className="flex flex-row justify-end items-center gap-1">
						<span className="text-sm">12</span>
						<Bookmark className="w-4 h-4 text-zinc-200" />
					</div>
				</Button>
			</div>
			<div className="flex flex-col">
				<Link
					href={`${post.blogSlug}/${post.slug}`}
					variant={{ underline: true }}
				>
					<h2>{post.title}</h2>
				</Link>
				<p className="text-sm">
					By{" "}
					<Link
						href={`/profiles/${post.author.id}`}
						variant={{ underline: true }}
					>
						{post.author.name}
					</Link>
				</p>
			</div>
		</div>
	)
}
