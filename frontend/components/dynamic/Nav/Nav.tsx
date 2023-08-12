import { Github } from "lucide-react"
import { GitHubLogoIcon } from "@radix-ui/react-icons"

import { Link } from "@/components/elements"

import Menu from "./Menu"

export default function Nav() {
	return (
		<nav className="mx-auto max-w-6xl px-2 py-3 flex flex-row justify-between items-center">
			<div className="flex flex-row gap-2 items-baseline">
				<Link href="/">
					<h1 className="font-semibold">Devy</h1>
				</Link>
				<span className="text-xs rounded-full px-2 py-0.5 bg-zinc-800">
					Pre-release
				</span>
			</div>

			<div className="flex flex-row border border-zinc-700 rounded-full bg-zinc-800">
				<Link href="/login">
					<div className="pl-2 pr-2 py-0.5 flex flex-row gap-2 items-center">
						<GitHubLogoIcon className="w-4 h-4" />
						<span className="text-sm font-medium">Sign in</span>
					</div>
				</Link>
				<Menu />
			</div>
		</nav>
	)
}
