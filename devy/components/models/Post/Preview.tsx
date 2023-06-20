import Link from "next/link"

import Post from "@/models/Post"

interface Props extends Post {}

export default function Preview(post: Props) {
	return (
		<div className="flex flex-row items-start gap-3">
			<div className="flex flex-col">
				<Link
					href={`/${post.blog.slug}/${post.slug}`}
					className="hover:underline transition-all"
				>
					<h1 className="font-medium">{post.title}</h1>
				</Link>
				<div className="flex flex-row gap-2 text-sm">
					<Link href={`/profiles/${post.author.username}`}>
						{post.author.displayName}
					</Link>
					<span className="text-slate-600">
						{new Date(post.published).toDateString()}
					</span>
				</div>
				<div className="flex flex-row gap-2 mt-1">
					{post.tags.map((tag, id) => (
						<Link
							href={`/tags/${tag}`}
							key={id}
							className="uppercase text-xs px-1.5 py-0.25 rounded-lg border border-slate-300"
						>
							{tag}
						</Link>
					))}
				</div>
			</div>
		</div>
	)
}
