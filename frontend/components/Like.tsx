import { useState } from "react"

import {
	HeartIcon as HeartIconOutline,
	HeartFilledIcon,
} from "@radix-ui/react-icons"

import { Button } from "@/components/elements"

interface Props {
	active: boolean
	initialIsLiked: boolean
	initialCount: number
	onLike: () => void
	onUnlike: () => void
}

export default function Like({ active, initialIsLiked, initialCount, onLike, onUnlike }: Props) {
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

	if (!active) return <Inactive count={count} />

	return (
		<Button className="flex flex-row gap-1 items-center w-14 px-2 py-0.5 rounded-md select-none group"
			onClick={onClick}
		>
			<HeartIcon isLiked={isLiked} />
			{count}
		</Button>
	)
}

const HeartIcon = ({ isLiked }: { isLiked: boolean }) => {
	if (isLiked)
		return <HeartFilledIcon className="text-zinc-300 h-3 aspect-square" />
	return <HeartIconOutline className="text-zinc-300 h-3 aspect-square" />
}

const Inactive = ({ count }: { count: number }) => (
	<span className="flex flex-row gap-1 items-center w-14 px-2 py-0.5 rounded-md select-none">
		<HeartIcon isLiked={false} />
		{count}
	</span>)

