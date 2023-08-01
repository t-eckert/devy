"use client"

import { useState } from "react"

import Feed from "@/models/Feed"
import Preview from "@/components/models/Post/Preview"
import Tab from "@/components/elements/Tab"
import { Frown } from "lucide-react"

interface Props {
	feeds: Feed[]
}

export default function HomeFeed({ feeds }: Props) {
	const [selectedFeed, setSelectedFeed] = useState<Feed>(feeds[0])
	const setFeed = (id: string) => {
		const feed = feeds.find((feed) => feed.id === id)
		if (feed) setSelectedFeed(feed)
	}

	return (
		<>
			<section className="flex flex-row sm:flex-col items-start gap-2">
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
			<section className="sm:px-4 w-full max-w-2xl flex flex-col items-start gap-4">
				{(!selectedFeed || selectedFeed.posts.length === 0) && (
					<div className="justify-self-center self-center my-8">
						<Frown className="w-8 h-8 mx-auto text-zinc-200 mb-2" />
						<span>No posts to show.</span>
					</div>
				)}
				{selectedFeed.posts &&
					selectedFeed.posts.map((post, i) => (
						<Preview key={i} {...post} />
					))}
			</section>
		</>
	)
}
