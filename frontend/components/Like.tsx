"use client"

import {
	HeartIcon as HeartIconOutline,
	HeartFilledIcon,
} from "@radix-ui/react-icons"

import { Button } from "@/components/elements"

interface Props {
	active: boolean
	isLiked: boolean
	count: number
}

export default function Like({ active, isLiked, count }: Props) {
	if (!active)
		return (
			<span className="flex flex-row gap-1 items-center w-14 px-2 py-0.5 rounded-md select-none">
				<HeartIcon isLiked={isLiked} />
				{count}
			</span>
		)

	return (
		<Button className="flex flex-row gap-1 items-center w-14 px-2 py-0.5 rounded-md select-none">
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
