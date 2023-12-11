import Post from "@/models/Post"
import Link from "@/components/link"
import Likes from "@/components/likes"
import RelativeDate from "@/components/relative-date"

interface Props {
	post: Post
}

export default function PostPreview({ post }: Props) {
	return (
		<section className={wrapper}>
			<div className="flex flex-col gap-1">
				<div className="ml-14 text-xs flex items-center justify-start gap-2">
					<Link
						href={{ pathname: `/${post.blogSlug}` }}
						variant={{ underline: false }}
						className="text-neutral hover:text-neutral-1 hover:dark:text-neutral+1"
					>
						{post.blogName}
					</Link>
					<RelativeDate
						date={post.createdAt}
						className="text-neutral select-none"
					/>
				</div>

				<div className="flex flex-row gap-2">
					<div className="w-12 flex flex-col items-start gap-1">
						<Likes postId={post.id} initialCount={post.likes} />
					</div>

					<div className="flex flex-col gap-1">
						<Link
							href={{
								pathname: `/${post.blogSlug}/${post.slug}`,
							}}
							className="font-medium text-zinc-50"
							variant={{ underline: false }}
						>
							{post.title}
						</Link>

						<div className="flex flex-row gap-2 items-baseline text-xs">
							<Link
								href={{
									pathname: `/profiles/${post.authorSlug}`,
								}}
								variant={{ underline: false }}
							>
								{post.authorName}
							</Link>
						</div>
					</div>
				</div>
			</div>
		</section>
	)
}

const wrapper = [
	"py-2",
	"px-4",
	"w-full",

	"dark:border-neutral-1",

	"rounded-md",
	"hover:shadow-md",
	"hover:border-neutral+1",
	"hover:dark:border-neutral-1",

	"bg-neutral+3",
	"dark:bg-neutral-2",

	"border",
	"hover:border",
	"border-neutral+3",
	"dark:border-neutral-2",

	"transition-all",
].join(" ")
