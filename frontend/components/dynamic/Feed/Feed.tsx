"use client"

import { useEffect, useState } from "react"

import Feed from "@/models/Feed"
import Post from "@/models/Post"
import Preview from "@/components/models/Post/Preview"
import { Tab } from "@/components/elements"
import { Frown } from "lucide-react"

interface Props {
	feeds: {
		feedMeta: Feed
		posts: Post[]
	}[]
}

export default function HomeFeed({ feeds }: Props) {
	const [selectedFeed, setSelectedFeed] = useState<Feed>(feeds[0].feedMeta)
	const setFeed = (id: string) => {
		const feed = feeds.find((feed) => feed.feedMeta.id === id)
		if (feed) setSelectedFeed(feed.feedMeta)
	}

	const [posts, setPosts] = useState<Post[]>(
		feeds.find((feed) => feed.feedMeta.id === selectedFeed.id)?.posts || []
	)

	useEffect(() => {
		const feed = feeds.find((feed) => feed.feedMeta.id === selectedFeed.id)
		if (feed) setPosts(feed.posts)
	}, [selectedFeed, feeds])

	// TODO: call to action when no posts: "Write one!"
	return (
		<>
			<section className="flex flex-row sm:flex-col sm:w-64 items-start gap-2">
				{feeds.map((feed) => (
					<Tab
						key={feed.feedMeta.id}
						id={feed.feedMeta.id}
						isSelected={selectedFeed.id === feed.feedMeta.id}
						setSelected={setFeed}
					>
						{feed.feedMeta.name}
					</Tab>
				))}
			</section>

			<section className="sm:px-4 w-full max-w-2xl flex flex-col items-start gap-4">
				{posts.length === 0 && (
					<div className="justify-self-center self-center my-8">
						<Frown className="w-8 h-8 mx-auto text-zinc-200 mb-2" />
						<span>No posts to show.</span>
					</div>
				)}
				{posts && posts.map((post, i) => <Preview key={i} {...post} />)}
			</section>
		</>
	)
}
