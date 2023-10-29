"use client"

import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { HamburgerMenuIcon } from "@radix-ui/react-icons"

import useSession, { SessionStore } from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import Link from "@/components/link"
import Button from "@/components/button"

export default function Menu() {
	const session = useStore(useSession, (state) => state)

	return (
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild>
				<button className="p-0.5" aria-label="Navigation menu">
					<HamburgerMenuIcon className="w-4 h-4" />
				</button>
			</DropdownMenu.Trigger>

			<DropdownMenu.Portal>
				<DropdownMenu.Content
					className="w-44 rounded-lg text-sm bg-zinc-800 border border-zinc-700 shadow-lg"
					sideOffset={5}
					align="end"
				>
					{session?.status === "logged-in" ? (
						<LoggedIn session={session} />
					) : (
						<LoggedOut />
					)}
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	)
}

const LoggedIn = ({ session }: { session: SessionStore }) => {
	return (
		<>
			<DropdownMenu.Item className="px-2 py-1 w-full">
				<Link href="/changelog" className="w-full">
					<span className="w-full">Changelog</span>
				</Link>
			</DropdownMenu.Item>

			<DropdownMenu.Item className="px-2 py-1 w-full">
				<Button
					className="w-full"
					onClick={() => session.clearSession()}
				>
					<span className="w-full">Sign Out</span>
				</Button>
			</DropdownMenu.Item>
		</>
	)
}

const LoggedOut = () => {
	return (
		<DropdownMenu.Item className="px-2 py-1 w-full">
			<Link href="/changelog" className="w-full">
				<span className="w-full">Changelog</span>
			</Link>
		</DropdownMenu.Item>
	)
}
