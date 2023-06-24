"use client"

import { useState } from "react"

import Feed from "@/models/Feed"
import List from "@/components/models/Post/List"
import Tab from "@/components/elements/Tab"

interface Props {
	feeds: Feed[]
}

export default function HomeFeed({ feeds }: Props) {
	const [selectedFeed, setSelectedFeed] = useState<Feed>(feeds[0])
	const setFeed = (id: number) => {
		const feed = feeds.find((feed) => feed.id === id)
		if (feed) setSelectedFeed(feed)
	}

	return (
		<>
			<section className="flex flex-col items-start gap-2">
				{feeds.map((feed) => (
					<Tab
						key={feed.id}
						id={feed.id}
						isSelected={selectedFeed.id === feed.id}
						setSelected={setFeed}
					>
						{feed.name}
					</Tab>
				))}
			</section>
			{selectedFeed.posts && <List posts={selectedFeed.posts} />}
		</>
	)
}
