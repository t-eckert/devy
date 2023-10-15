import { useQuery } from "@tanstack/react-query"
import { useState } from "react"

import Preview from "@/components/Preview"
import Content from "./Content"
import fetchContent from "./fetchContent"
import { Button } from "@/components/elements"
import useSession from "@/lib/auth"

interface Props {
	initialContent: Content
}

const pageSize = 15

function Feed({ initialContent: content }: Props) {
	const { status } = useSession()

	const [page, setPage] = useState<number>(0)

	const { data } = useQuery({
		queryKey: [content.feed.id, page],
		queryFn: () => fetchContent(content.feed.id, page, pageSize),
		initialData: content,
	})

	const userLikes = new Set<string>()

	return (
		<section className="w-full mx-auto max-w-xl flex flex-col gap-4">
			{data &&
				data.posts &&
				data.posts.map((post, i) => (
					<Preview
						key={i}
						{...post}
						session={status}
						isLiked={userLikes.has(post.id)}
					/>
				))}
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
