import Post from "@/models/Post"
import Link from "@/components/link"
import Likes from "@/components/likes"

import Title from "./post-title"

interface Props {
	post: Post
}

export default function PostPreview({ post }: Props) {
	return (
		<section className="flex flex-row gap-2 items-start">
			<div className="flex flex-col items-start gap-1">
				<Likes postId={post.id} initialCount={post.likes} />
			</div>

			<div className="flex flex-col gap-1">
				<Title
					title={post.title}
					url={`${post.blogSlug}/${post.slug}`}
				/>

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
