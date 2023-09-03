import { GitHubLogoIcon } from "@radix-ui/react-icons"

import { Link } from "@/components/elements"

import Menu from "./Menu"


export default function Nav() {
	return (
		<nav className="flex flex-row border border-zinc-700 rounded-full bg-zinc-800">
			<Link href="/api/auth/login?redirect=/" prefetch={false}>
				<div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center border-r border-r-zinc-700 hover:bg-zinc-700 transition-all">
					<GitHubLogoIcon className="w-4 h-4" />
					<span className="text-sm font-medium">Sign in</span>
				</div>
			</Link>
			<Menu />
		</nav>
	)
}
