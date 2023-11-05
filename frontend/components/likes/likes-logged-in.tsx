import { HeartIcon, HeartFilledIcon } from "@radix-ui/react-icons"

import Button from "@/components/button"
import Counter from "@/components/counter"

export default function LikesLoggedIn({
	count,
	isLiked,
	onClick,
}: {
	count: number
	isLiked: boolean
	onClick: () => void
}) {
	return (
		<Button onClick={onClick} variant={{ intent: "tertiary" }}>
			<div className="flex flex-row gap-0.5 items-center justify-center rounded-md select-none group">
				<span>
					{isLiked ? (
						<HeartFilledIcon className={heartIcon} />
					) : (
						<HeartIcon className={heartIcon} />
					)}
				</span>
				<span className="flex flex-col items-start text-neutral-darker dark:text-neutral-lighter">
					<Counter count={count} />
				</span>
			</div>
		</Button>
	)
}

const heartIcon = [
	"text-neutral-darker",
	"dark:text-neutral-lighter",
	"h-4",
	"aspect-square",
	"group-hover:-translate-y-0.5",
	"group-hover:scale-110",
	"group-hover:rotate-12",
	"transition-all",
].join(" ")
