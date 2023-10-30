import { useState } from "react"

import { HeartIcon, HeartFilledIcon } from "@radix-ui/react-icons"

import Button from "@/components/button"
import Counter from "@/components/counter"

interface Props {
	active: boolean
	initialIsLiked: boolean
	initialCount: number
	onLike: () => void
	onUnlike: () => void
}

export default function Likes({
	active,
	initialIsLiked,
	initialCount,
	onLike,
	onUnlike,
}: Props) {
	const [isLiked, setIsLiked] = useState(initialIsLiked)
	const [count, setCount] = useState(initialCount)

	const onClick = () => {
		if (isLiked) {
			onUnlike()
			setIsLiked(false)
			setCount(count - 1)
		} else {
			onLike()
			setIsLiked(true)
			setCount(count + 1)
		}
	}

	// The Likes component is non-interactive if the user is not logged in.
	if (!active) {
		return (
			<div className="flex flex-row gap-1 items-center justify-center px-2 py-0.5 rounded-md select-none group">
				<span>
					<HeartIcon className="text-neutral-low dark:text-neutral-light h-4 aspect-square" />
				</span>
				<span className="text-sm font-medium w-7 flex flex-col items-end">
					<Counter count={count} />
				</span>
			</div>
		)
	}

	return (
		<Button onClick={onClick} variant={{ intent: "secondary" }}>
			<div className="flex flex-row gap-1 items-center justify-center rounded-md select-none group">
				<span>
					{isLiked ? (
						<HeartFilledIcon className="text-neutral-low dark:text-neutral-light h-4 aspect-square" />
					) : (
						<HeartIcon className="text-neutral-low dark:text-neutral-light h-4 aspect-square" />
					)}
				</span>
				<span className="w-7 flex flex-col items-end">
					<Counter count={count} />
				</span>
			</div>
		</Button>
	)
}
