import Link from "@/components/link"
import Logomark from "@/components/logomark"

import Nav from "@/components/nav"

export default function Header() {
	return (
		<header className="border-b border-b-neutral+1 dark:border-b-neutral-1">
			<div className="mx-auto max-w-6xl px-3 py-3 flex flex-row justify-between items-center">
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

				<Nav />
			</div>
		</header>
	)
}
