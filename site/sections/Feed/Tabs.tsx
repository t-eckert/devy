"use client"

import { FeedState, useFeed } from "./useFeed"

const Tabs: React.FC = () => {
	const { feeds, currentFeed } = useFeed()

	return (
		<nav className="flex flex-row gap-4">
			{feeds.map((feed, index) => {
				return <div key={index}>{feed.name}</div>
			})}
		</nav>
	)
}

export default Tabs
