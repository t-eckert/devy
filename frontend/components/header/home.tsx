import Link from "@/components/elements/link"
import Logomark from "@/components/elements/logomark"

export default function Home() {
	return (
		<div className="flex flex-row gap-2 items-baseline">
			<Link
				href="/"
				variant={{ underline: false, styled: false }}
				className="px-0.5 -ml-0.5"
			>
				<Logomark />
			</Link>
			<span className="text-xs font-medium px-2 py-0.5 rounded-full text-neutral-1 bg-neutral+1 dark:text-neutral+2 dark:bg-neutral-1 select-none">
				Preview
			</span>
		</div>
	)
}
