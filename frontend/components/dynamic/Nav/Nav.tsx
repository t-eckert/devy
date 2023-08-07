import { Github } from "lucide-react"

import Link from "@/components/elements/Link"

export default function Nav() {
	return (
		<nav className="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between">
			<div className="flex flex-row gap-2 items-baseline">
				<Link href="/">
					<h1 className="font-semibold">Devy</h1>
				</Link>
				<span className="text-xs rounded-full px-2 py-0.5 bg-zinc-800">
					Pre-release
				</span>
			</div>

			<div>
				<Link href="/login">
					<div className="flex flex-row gap-2 items-center">
						<Github className="w-4 h-4" />
						<span className="text-sm font-medium">Sign in</span>
					</div>
				</Link>
			</div>
		</nav>
	)
}
