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
			<div className="grid grid-rows-3 grid-cols-[max-content_1fr] gap-x-4">
				<Link
					href={{
						pathname: `/${post.blogSlug}/${post.slug}`,
					}}
					className="font-medium text-zinc-50 col-start-2 col-span-3 row-start-2"
					variant={{ underline: false }}
				>
					{post.title}
				</Link>
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
						variant={{ underline: false }}
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
							pathname: `/profiles/${post.authorSlug}`,
						}}
						variant={{ underline: false }}
						aria-label={`View ${post.authorName}'s profile`}
					>
						{post.authorName}
					</Link>
				</div>
			</div>
		</section>
	)
}

const wrapper = [
	"py-1",
	"px-4",
	"w-full",

	"dark:border-neutral-1",

	"rounded-md",
	"hover:shadow-md",
	"hover:border-neutral+1",
	"hover:dark:border-neutral-1",
	"focus-within:shadow-md",
	"focus-within:border-neutral+1",
	"focus-within:dark:border-neutral-1",

	"bg-neutral+3",
	"dark:bg-neutral-2",

	"border",
	"hover:border",
	"border-neutral+3",
	"dark:border-neutral-2",

	"transition-all",
].join(" ")
