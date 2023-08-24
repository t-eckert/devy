"use client"

import Post from "@/models/Post"
import { Link } from "@/components/elements"

export default function Preview(post: Post) {
	return (
		<div className="flex flex-col gap-2">
			<div className="flex flex-row items-start gap-1">
				<div className="flex flex-col">
					<Link
						href={`${post.blogSlug}/${post.slug}`}
						variant={{ underline: true }}
					>
						<h2>{post.title}</h2>
					</Link>
					<span className="text-sm text-zinc-300 flex flex-row items-center gap-2">
						<Link
							href={`/${post.blogSlug}`}
							variant={{ underline: true }}
						>
							{post.blogName}
						</Link>
						<span>/</span>
						<Link
							href={`/profiles/${post.authorSlug}`}
							variant={{ underline: true }}
						>
							{post.authorName}
						</Link>
						<span>
							{new Date(post.createdAt).toLocaleDateString(
								"en-US",
								{
									month: "long",
									day: "numeric",
									year: "numeric",
								}
							)}
						</span>
					</span>
				</div>
			</div>
		</div>
	)
}
