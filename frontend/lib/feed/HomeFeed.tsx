"use client"

import { Dispatch, SetStateAction, useState } from "react"

import { Frown } from "lucide-react"

import Preview from "@/components/models/Post/Preview"
import { Tab } from "@/components/elements"

import type { Feed, Post } from "@/models"
import Json from "@/components/debugging/Json"

export interface FeedContent {
	metadata: Feed
	status: "loaded" | "loading" | "error"
	posts: Post[]
	offset: number
	pageSize: number
}

interface Props {
	// NOTE: This datastructure would be more efficient as a map, but I am passing from server to client so I need to use an array.
	feeds: FeedContent[]
	defaultSelected?: string
}

export default function HomeFeed({ feeds, defaultSelected }: Props) {
	const [selected, setSelected] = useState<string>(
		defaultSelected || feeds[0].metadata.id
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
	feeds: FeedContent[]
	selected: string
	setSelected: Dispatch<SetStateAction<string>>
}) => {
	return (
		<div className="flex flex-row sm:flex-col sm:w-64 items-state gap-2">
			{feeds.map((feed) => (
				<Tab
					key={feed.metadata.id}
					id={feed.metadata.id}
					isSelected={feed.metadata.id === selected}
					setSelected={setSelected}
				>
					{feed.metadata.name}
				</Tab>
			))}
		</div>
	)
}

const List = ({ feeds, selected }: { feeds: FeedContent[]; selected: string }) => {
	const feed = feeds.find((feed) => feed.metadata.id === selected)

	if (!feed) {
		return <div>Unable to find feed {selected}</div>
	}

	return (
		<div className="sm:px-4 w-full max-w-2xl flex flex-col items-start gap-4">
			{feed.posts.length === 0 && (
				<div className="justify-self-center self-center my-8">
					<Frown className="w-8 h-8 mx-auto text-zinc-200 mb-2" />
					<span>No posts to show.</span>
				</div>
			)}
			{feed.posts &&
				feed.posts.map((post, i) => <Preview key={i} {...post} />)}
		</div>
	)
}
