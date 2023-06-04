import Feed from "@/models/Feed"

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
		<section className="w-full max-w-2xl">
			{feed.posts.map((post, id) => (
				<div key={id}>{post.title}</div>
			))}
		</section>
	)
}
