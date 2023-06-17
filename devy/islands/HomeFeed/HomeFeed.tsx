"use client"

import { useState } from "react"

import { Preview } from "@/components/models/Post"
import Tab from "@/components/elements/Tab"
import Feed from "@/models/Feed"
import useHomeFeed from "./useHomeFeed"

interface Props {
	feeds: Feed[]
}

export default function HomeFeed({ feeds }: Props) {
	const ctx = useHomeFeed()

	const [selectedFeed, setSelectedFeed] = useState<number>(1)

	return (
		<>
			<section className="flex flex-col items-start gap-2">
				{feeds.map((feed) => (
					<Tab
						key={feed.id}
						id={feed.id}
						isSelected={selectedFeed === feed.id}
						setSelected={setSelectedFeed}
					>
						{feed.name}
					</Tab>
				))}
			</section>
			<section className="w-full max-w-2xl flex flex-col gap-4">
				{feeds
					.find((feed) => feed.id === selectedFeed)
					?.posts.map((post, i) => (
						<Preview key={i} {...post} />
					))}
			</section>
		</>
	)
}
