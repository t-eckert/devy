"use client"

import { HeartIcon, HeartFilledIcon } from "@radix-ui/react-icons"

import Post from "@/models/Post"
import { Button, Link } from "@/components/elements"
import Like from "@/components/Like"

import api from "@/lib/api"
import { SessionStatus } from "@/lib/auth"

interface Props extends Post {
	session: SessionStatus
	isLiked: boolean
}

export default function Preview({
	id,
	title,
	slug,
	blogSlug,
	authorName,
	authorSlug,
	blogName,
	likes,
	session,
	isLiked,
}: Props) {
	const user = "e2f0fa7e-4517-4ac8-bbc6-73067d3feed4"

	const like = async () => {
		console.log("like")
		await api.post(`/posts/${id}/likes`, 10, user)
	}

	return (
		<section className="flex flex-row gap-2 items-start">
			<div className="flex flex-col items-start gap-1">
				<Like
					active={session === "logged-in"}
					isLiked={isLiked}
					count={likes}
				/>
			</div>

			<div className="flex flex-row items-start gap-1">
				<div className="flex flex-col gap-1">
					<Link
						href={`${blogSlug}/${slug}`}
						className="font-medium text-zinc-50"
					>
						{title}
					</Link>

					<div className="mb-2 flex flex-row gap-2 items-baseline text-sm">
						<Link href={`/profiles/${authorSlug}`}>
							{authorName}
						</Link>
						<Link href={`/${blogSlug}`}>{blogName}</Link>
					</div>
				</div>
			</div>
		</section>
	)
}
