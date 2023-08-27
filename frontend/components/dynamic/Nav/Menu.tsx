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
					<DropdownMenu.Item className="px-2 w-full">
						<Link href="/changelog" className="w-full">
							<span className="w-full">Changelog</span>
						</Link>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	)
}
