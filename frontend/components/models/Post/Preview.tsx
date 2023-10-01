"use client"

import Post from "@/models/Post"
import Date from "@/components/elements/Date"
import { Link } from "@/components/elements"

export default function Preview(post: Post) {
	return (
		<div className="flex flex-col gap-2">
			<div className="flex flex-row items-start gap-1">
				<div className="flex flex-col gap-1">
					<Link
						href={`${post.blogSlug}/${post.slug}`}
						variant={{ underline: false }}
						className="font-medium"
					>
						<h2>{post.title}</h2>
					</Link>
					<div className="mb-2 flex flex-row gap-2 items-baseline text-sm">
						<div className="bg-zinc-800 text-xs flex items-center justify-center px-2 py-1 rounded-full">
							<Date date={post.createdAt} />
						</div>
						<Link
							href={`/profiles/${post.authorSlug}`}
							variant={{ underline: false }}
						>
							{post.authorName}
						</Link>
						<span>/</span>
						<Link
							href={`/${post.blogSlug}`}
							variant={{ underline: false }}
						>
							{post.blogName}
						</Link>
					</div>
				</div>
			</div>
		</div>
	)
}
