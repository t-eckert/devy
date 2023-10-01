"use client"

import React, { Dispatch, SetStateAction, useState } from "react"
import { QueryClientProvider, QueryClient } from "@tanstack/react-query"

import { Frown } from "lucide-react"

import Preview from "@/components/models/Post/Preview"
import { Button } from "@/components/elements"
import Tab from "./Tab"

import type { Feed, Post } from "@/models"

export interface FeedContent {
	metadata: Feed
	status: "loaded" | "loading" | "error"
	posts: Post[]
	page: number
	pageSize: number
}

interface Props {
	// NOTE: This datastructure would be more efficient as a map, but I am passing from server to client so I need to use an array.
	feeds: FeedContent[]
	defaultSelected: string
}

// HomeFeed
export default function HomeFeed({ feeds, defaultSelected }: Props) {
	const [selected, setSelected] = useState<string>(defaultSelected)

	return (
		<section className="w-full flex flex-row gap-8">
			<Provider>
				<Tabs
					feeds={feeds}
					selected={selected}
					setSelected={setSelected}
				/>
				<div className="sm:px-4 w-full max-w-2xl flex flex-col items-start gap-4">
					<List feeds={feeds} selected={selected} />
					<Pagination feeds={feeds} selected={selected} />
				</div>
			</Provider>
		</section>
	)
}

const Provider = ({ children }: React.PropsWithChildren) => {
	return <div>{children}</div>
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
		<div className="flex flex-row mb-4 sm:w-64 items-state gap-2">
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

const List = ({
	feeds,
	selected,
}: {
	feeds: FeedContent[]
	selected: string
}) => {
	const feed = feeds.find((feed) => feed.metadata.id === selected)

	if (!feed) {
		return <div>Unable to find feed {selected}</div>
	}

	return (
		<>
			{feed.posts.length === 0 && (
				<div className="justify-self-center self-center my-8">
					<Frown className="w-8 h-8 mx-auto text-zinc-200 mb-2" />
					<span>No posts to show.</span>
				</div>
			)}
			{feed.posts &&
				feed.posts.map((post, i) => <Preview key={i} {...post} />)}
		</>
	)
}

const Pagination = ({
	feeds,
	selected,
}: {
	feeds: FeedContent[]
	selected: string
}) => {
	return (
		<div className="mt-4 w-full flex flex-row justify-between">
			<Button>Previous</Button>
			<Button>Next</Button>
		</div>
	)
}
