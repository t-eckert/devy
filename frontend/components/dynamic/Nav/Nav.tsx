import { GitHubLogoIcon } from "@radix-ui/react-icons"

import { Link } from "@/components/elements"

import Menu from "./Menu"

const signInImplemented = false

export default function Nav() {
	return (
		<nav className="flex flex-row border border-zinc-700 rounded-full bg-zinc-800">
			{signInImplemented && (
				<Link href="/login">
					<div className="pl-2 pr-2 py-0.5 flex flex-row gap-2 items-center">
						<GitHubLogoIcon className="w-4 h-4" />
						<span className="text-sm font-medium">Sign in</span>
					</div>
				</Link>)}
			<Menu />
		</nav>
	)
}
