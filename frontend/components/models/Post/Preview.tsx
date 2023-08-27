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
						variant={{ underline: true }}
					>
						<h2>{post.title}</h2>
					</Link>
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
						<Link
							href={`/${post.blogSlug}`}
							variant={{ underline: true }}
						>
							{post.blogName}
						</Link>
					</div>
				</div>
			</div>
		</div>
	)
}
