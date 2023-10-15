import Post from "@/models/Post"
import { Link } from "@/components/elements"
import Like from "@/components/Like"

import { SessionStatus } from "@/lib/auth"
import Title from "./Title"

interface Props extends Post {
	session: SessionStatus
	isLiked: boolean
	onLike: () => void
	onUnlike: () => void
}

export default function Preview({
	session,
	isLiked,
	onLike,
	onUnlike,
	...post
}: Props) {
	return (
		<section className="flex flex-row gap-2 items-start">
			<div className="flex flex-col items-start gap-1">
				<Like
					active={session === "logged-in"}
					initialIsLiked={isLiked}
					initialCount={post.likes}
					onLike={onLike}
					onUnlike={onUnlike}
				/>
			</div>

			<div className="flex flex-col gap-1">
				<Title title={post.title} url={`${post.blogSlug}/${post.slug}`} />

				<div className="mb-2 flex flex-row gap-2 items-baseline text-sm">
					<Link href={`/profiles/${post.authorSlug}`}>
						{post.authorName}
					</Link>
					<Link href={`/${post.blogSlug}`}>{post.blogName}</Link>
				</div>
			</div>
		</section>
	)
}
