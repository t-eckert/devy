import Preview from "components/Preview"
import { PostMetadata } from "interfaces"
import { useState } from "react"

interface FeedMetadata {
	id: string
	name: string
}

const feeds: FeedMetadata[] = [
	{ id: "37e24b45-e5d2-456e-94ef-11f2c64ef17f", name: "Popular" },
	{ id: "aee0f9e9-a028-4c8b-a453-dbdbcf309fc0", name: "New" },
]

const postsMetadata: PostMetadata[] = [
	{
		title:
			"They Didn't Tell Me I Had to Eat the Two Pizzas: Stories of an IC Turned Manager",
		author: {
			id: "95781875-ba3f-417c-9b4b-dec6b6aea354",
			name: "Demari Williams",
		},
		path: "/d-williams/they-didnt-tell-me",
		tags: ["management"],
		likes: 300,
	},
]

const Feed: React.FC = () => {
	const [feedId, setFeedId] = useState<string>(
		"37e24b45-e5d2-456e-94ef-11f2c64ef17f"
	)

	return (
		<section className="flex flex-col">
			<div className="flex flex-row justify-between">
				<ul className="flex flex-row gap-2 font-medium">
					{feeds.map((feed) => (
						<li
							key={feed.id}
							className={[
								"cursor-pointer",
								feedId === feed.id ? "underline" : "",
							].join(" ")}
							onClick={() => setFeedId(feed.id)}
						>
							{feed.name}
						</li>
					))}
				</ul>
			</div>
			{postsMetadata.map((postMetadata, index) => (
				<Preview key={index} postMetadata={postMetadata} />
			))}
		</section>
	)
}

export default Feed
