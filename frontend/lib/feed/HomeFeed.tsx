"use client"

import { Dispatch, SetStateAction, useState } from "react"

import { Frown } from "lucide-react"

import Preview from "@/components/models/Post/Preview"
import { Tab } from "@/components/elements"

import type { Feed, Post } from "@/models"
import Json from "@/components/debugging/Json"

type Feeds = Map<string, FeedContent>

export interface FeedContent {
	metadata: Feed
	status: "loaded" | "loading" | "error"
	posts: Post[]
	offset: number
	pageSize: number
}

interface Props {
	feeds: Feeds
	defaultSelected?: string
}

export default function HomeFeed({ feeds, defaultSelected }: Props) {
	const [selected, setSelected] = useState<string>(
		defaultSelected || feeds.keys().next().value.id
	)

	return (
		<section className="w-full flex flex-row gap-8">
			<Tabs feeds={feeds} selected={selected} setSelected={setSelected} />
			<List feeds={feeds} selected={selected} />
		</section>
	)
}

const Tabs = ({
	feeds,
	selected,
	setSelected,
}: {
	feeds: Feeds
	selected: string
	setSelected: Dispatch<SetStateAction<string>>
}) => {
	return (
		<div className="flex flex-row sm:flex-col sm:w-64 items-state gap-2">
			{Array.from(feeds).map(([id, feed]) => (
				<Tab
					key={id}
					id={id}
					isSelected={id === selected}
					setSelected={setSelected}
				>
					{feed.metadata.name}
				</Tab>
			))}
		</div>
	)
}

const List = ({ feeds, selected }: { feeds: Feeds; selected: string }) => {
	return <div>List</div>
	// const feed = feeds.get(selected)

	// if (!feed) {
	// 	return <div>Unable to find feed {selected}</div>
	// }

	// return (
	// 	<div className="sm:px-4 w-full max-w-2xl flex flex-col items-start gap-4">
	// 		{feed.posts.length === 0 && (
	// 			<div className="justify-self-center self-center my-8">
	// 				<Frown className="w-8 h-8 mx-auto text-zinc-200 mb-2" />
	// 				<span>No posts to show.</span>
	// 			</div>
	// 		)}
	// 		{feed.posts &&
	// 			feed.posts.map((post, i) => <Preview key={i} {...post} />)}
	// 	</div>
	// )
}
