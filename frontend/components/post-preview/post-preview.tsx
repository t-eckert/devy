import Post from "@/models/Post"
import Link from "@/components/link"
import Likes from "@/components/likes"

import Title from "./post-title"

interface Props {
	post: Post
}

export default function PostPreview({ post }: Props) {
	return (
		<section className={wrapper}>
			<div className="flex flex-col items-start gap-1">
				<Likes postId={post.id} initialCount={post.likes} />
			</div>

			<div className="flex flex-col gap-1">
				<Title
					title={post.title}
					url={`${post.blogSlug}/${post.slug}`}
				/>

				<div className="flex flex-row gap-2 items-baseline text-xs">
					<Link
						href={`/profiles/${post.authorSlug}`}
						variant={{ underline: false }}
					>
						{post.authorName}
					</Link>
					<Link
						href={`/${post.blogSlug}`}
						variant={{ underline: false }}
					>
						{post.blogName}
					</Link>
				</div>
			</div>
		</section>
	)
}

const wrapper = [
	"flex",
	"flex-row",
	"gap-4",
	"items-start",
	"py-2",
	"px-4",
	"border-neutral-darker",
	"rounded-md",
	"hover:shadow-md",
	"hover:border-neutral-medium",
	"bg-neutral-lightest",
	"dark:bg-neutral-darker",
	"border",
	"hover:border",
	"transition-all",
].join(" ")
