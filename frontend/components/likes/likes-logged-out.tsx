import { HeartIcon } from "@radix-ui/react-icons"
import Counter from "@/components/counter"

export default function LikesLoggedOut({ count }: { count: number }) {
	return (
		<div className="flex flex-row gap-0.5 items-center justify-center px-1 py-0.5 rounded-md select-none group">
			<span>
				<HeartIcon className="text-neutral-darker dark:text-neutral-lighter h-4 aspect-square" />
			</span>
			<span className="text-sm flex flex-col items-start text-neutral-darker dark:text-neutral-lighter">
				<Counter count={count} />
			</span>
		</div>
	)
}
