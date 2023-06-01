"use client"

import Preview from "components/Preview"

import { useFeed } from "./useFeed"

const List = () => {
	const { feeds, currentFeed } = useFeed()

	return (
		<div className="flex flex-col gap-4">
			{JSON.stringify(feeds.filter(({ name }) => name === currentFeed))}
		</div>
	)
}

export default List
