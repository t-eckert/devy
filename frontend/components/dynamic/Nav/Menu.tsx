"use client"

import { Button } from "@/components/elements"
import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { HamburgerMenuIcon } from "@radix-ui/react-icons"

export default function Menu() {
	return (
		<DropdownMenu.Root>

			<DropdownMenu.Trigger asChild>
				<Button className="pl-2 pr-3 py-0.5 border-l border-zinc-700 hover:bg-zinc-700 rounded-r-full" aria-label="Navigation menu">
					<HamburgerMenuIcon className="w-4 h-4" />
				</Button>
			</DropdownMenu.Trigger>

			<DropdownMenu.Portal>
				<DropdownMenu.Content>
					<DropdownMenu.Item >
						Changelog
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	)
}
