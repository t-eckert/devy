"use client"

import { useQuery } from "@tanstack/react-query"
import { useState } from "react"

import fetchContent from "./fetchContent"
import Button from "@/components/button"
import Posts from "@/components/posts"
import { Feed, Post } from "@/models"

interface Content {
	feed: Feed
	posts: Post[]
}

interface Props {
	initialContent: Content
}

const pageSize = 15

function Feed({ initialContent }: Props) {
	const [page, setPage] = useState<number>(0)

	const { data } = useQuery({
		queryKey: [initialContent.feed.id, page],
		queryFn: () => fetchContent(initialContent.feed.id, page, pageSize),
		initialData: initialContent,
	})

	return (
		<section className="w-full mx-auto max-w-xl flex flex-col gap-4">
			<Posts posts={data?.posts || []} />
			<div className="pt-2 border-t border-t-zinc-700 w-full flex flex-row-reverse justify-between">
				<Button
					onClick={() => setPage(page + 1)}
					variant={{ intent: "secondary" }}
				>
					Next
				</Button>
				{page != 0 && (
					<Button
						onClick={() => setPage(page - 1)}
						variant={{ intent: "secondary" }}
					>
						Previous
					</Button>
				)}
			</div>
		</section>
	)
}

export default function FeedOrNotFound(props: Partial<Props>) {
	if (props.initialContent === undefined) {
		return <div>Cannot find feed</div>
	}

	return <Feed {...(props as Props)} />
}
