"use client"

import React, { useState } from "react"
import { QueryClientProvider, QueryClient } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

import type Content from "./Content"
import Feed from "./Feed"
import Tabs from "./Tabs"

const queryClient = new QueryClient()

interface Props {
	// NOTE: This datastructure would be more efficient as a map, but I am passing from server to client so I need to use an array.
	content: Content[]
	defaultSelected: string
}

function Feeds({ content, defaultSelected }: Props) {
	const [selected, setSelected] = useState<string>(defaultSelected)

	return (
		<section className="w-full flex flex-col md:flex-row items-start gap-4">
			<Tabs
				feeds={content.map(({ feed }) => feed)}
				selected={selected}
				setSelected={setSelected}
			/>
			<Feed
				initialContent={content.find(
					(content) => content.feed.id === selected
				)}
			/>
		</section>
	)
}

const FeedsWithProvider = (props: Props) => {
	return (
		<QueryClientProvider client={queryClient}>
			<Feeds {...props} />
			<ReactQueryDevtools initialIsOpen={false} />
		</QueryClientProvider>
	)
}

export default FeedsWithProvider
