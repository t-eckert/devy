"use client"

import { Link } from "@/components/elements"
import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { HamburgerMenuIcon } from "@radix-ui/react-icons"

export default function Menu() {
	return (
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild>
				<button
					className="pl-2 pr-2 py-0.5 border-zinc-700 hover:bg-zinc-700 rounded-full"
					aria-label="Navigation menu"
				>
					<HamburgerMenuIcon className="w-4 h-4" />
				</button>
			</DropdownMenu.Trigger>

			<DropdownMenu.Portal>
				<DropdownMenu.Content
					className="w-44 rounded-lg bg-zinc-800 border border-zinc-700 shadow-lg"
					sideOffset={5}
					align="end"
				>
					<DropdownMenu.Item className="px-2 pt-1">
						<Link href="/profile">Profile</Link>
					</DropdownMenu.Item>
					<DropdownMenu.Item className="px-2">
						<Link href="/changelog">Changelog</Link>
					</DropdownMenu.Item>
					<DropdownMenu.Item className="px-2 pb-1">
						<Link href="/sign-out">Sign out</Link>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	)
}
