import Feed from "@/models/Feed"
import PostPreview from "@/components/elements/PostPreview"

interface Props {
	feed?: Feed
}

export default function Feed({ feed }: Props) {
	if (!feed)
		return (
			<section className="w-full max-w-2xl">
				<p>Unable to find feed</p>
			</section>
		)

	return (
		<section className="w-full max-w-2xl flex flex-col gap-4">
			{feed.posts.map((post, id) => (
				<PostPreview key={id} post={post} />
			))}
		</section>
	)
}
