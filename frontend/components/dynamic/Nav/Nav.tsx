"use client"

import { GitHubLogoIcon } from "@radix-ui/react-icons"

import { Link } from "@/components/elements"

import Menu from "./Menu"
import useSession from "@/auth/useSession"
import type Session from "@/auth/Session"
import Avatar from "@/components/elements/Avatar"

export default function Nav() {
	const { session } = useSession()

	return (
		<nav className="flex flex-row border border-zinc-700 rounded-full bg-zinc-800">
			{session ? <Token session={session} /> : <Login />}
			<Menu />
		</nav>
	)
}

const Login = () => (
	<Link href="/api/auth/login" prefetch={false}>
		<div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center border-r border-r-zinc-700 hover:bg-zinc-700 transition-all">
			<GitHubLogoIcon className="w-4 h-4" />
			<span className="text-sm font-medium">Sign in</span>
		</div>
	</Link>
)

const Token = ({ session }: { session: Session }) => (
	<Link href={`/profiles/${session.user.username}`}>
		<div className="pl-2 pr-2 py-0.5 flex flex-row rounded-l-full gap-2 items-center border-r border-r-zinc-700 hover:bg-zinc-700 transition-all">
			<Avatar
				displayName={session.profile.displayName}
				avatarUrl={session.profile.avatarUrl}
				className="w-4 h-4"
			/>
			<span className="text-sm font-medium">
				{session.profile.displayName}
			</span>
		</div>
	</Link>
)
