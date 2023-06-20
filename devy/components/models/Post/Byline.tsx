import Link from "next/link"
import { Heart, Bookmark } from "lucide-react"

import Post from "@/models/Post"

interface Props extends Post {}

export default function Byline(post: Props) {
	return (
		<section>
			<div className="flex flex-col">
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
			<div className="flex flex-row text-slate-500">
				<div className="flex flex-row gap-1 items-center">
					<span className="text-sm text-slate-700">{post.likes}</span>
					<Heart className="w-4 aspect-square" />
				</div>
				<div>
					<Bookmark className="w-4 aspect-square" />
				</div>
			</div>
		</section>
	)
}