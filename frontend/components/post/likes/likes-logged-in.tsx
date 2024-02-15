import { HeartIcon, HeartFilledIcon } from "@radix-ui/react-icons"

import Button from "@/components/elements/button"
import Counter from "@/components/utils/counter"

export default function LikesLoggedIn({
	count,
	title,
	isLiked,
	onClick,
}: {
	count: number
	title: string
	isLiked: boolean
	onClick: () => void
}) {
	return (
		<Button
			onClick={onClick}
			variant={{ intent: "tertiary" }}
			aria-label={`Like counter for ${title} with count ${count}`}
		>
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
	"h-4",
	"aspect-square",

	"text-neutral-darker",
	"dark:text-neutral-lighter",

	"group-hover:-translate-y-0.5",
	"group-hover:scale-110",
	"group-hover:rotate-12",
	"group-hover:text-red-primary",

	"transition-all",
].join(" ")