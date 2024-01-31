import Post from "@/models/Post"
import Link from "@/components/elements/link"
import Likes from "@/components/post/likes"
import RelativeDate from "@/components/utils/relative-date"

interface Props {
	post: Post
}

export default function Preview({ post }: Props) {
	return (
		<section className={wrapper}>
			<div className="grid grid-rows-3 grid-cols-[max-content_1fr] gap-x-8">
				<div className="col-start-2 col-span-3 row-start-2">
					<Link
						href={{
							pathname: `/${post.blogSlug}/${post.slug}`,
						}}
						className="font-medium text-zinc-50"
						variant={{ underline: false, styled: false }}
					>
						{post.title}
					</Link>
				</div>
				<div className="w-8 col-start-1 row-start-2">
					<Likes
						postId={post.id}
						title={post.title}
						initialCount={post.likes}
					/>
				</div>

				<div className="text-xs flex items-center justify-start gap-2 col-start-2 col-span-3">
					<Link
						href={{ pathname: `/${post.blogSlug}` }}
						variant={{ underline: false, styled: false }}
						className="text-neutral hover:text-neutral-1 hover:dark:text-neutral+1"
						aria-label={`View blog ${post.blogName}`}
					>
						{post.blogName}
					</Link>
					<RelativeDate
						date={post.createdAt}
						className="text-neutral select-none"
					/>
				</div>

				<div className="flex flex-row gap-2 col-start-2 col-span-3 row-start-3 items-baseline text-xs">
					<Link
						href={{
							pathname: `/profiles/${post.authorUsername}`,
						}}
						variant={{ underline: false, styled: false }}
						aria-label={`View ${post.authorName}'s profile`}
					>
						{post.authorName}
					</Link>
				</div>
			</div>
		</section>
	)
}

const wrapper = ["py-1", "px-4", "w-full", "transition-all"].join(" ")
