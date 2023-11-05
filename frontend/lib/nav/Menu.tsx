"use client"

import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { HamburgerMenuIcon } from "@radix-ui/react-icons"

import Link from "@/components/link"
import Button from "@/components/button"

import useSession, { SessionStore } from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"

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
					className="py-1 px-2 rounded-md border shadow-md border-neutral-darker dark:border-neutral-medium bg-neutral-lightest dark:bg-neutral-darker"
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
