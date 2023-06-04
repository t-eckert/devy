"use client"

import Link from "next/link"
import { Menu as MenuIcon } from "lucide-react"
import { Menu } from "@headlessui/react"

export default function Nav() {
	return (
		<div className="flex items-center justify-center">
			<Menu>
				<Menu.Button>
					<MenuIcon className="w-5 aspect-square" />
				</Menu.Button>
				<Menu.Items className="absolute flex flex-col top-8 right-3 py-2 bg-white rounded-lg shadow-xl border border-slate-200 w-44 text-sm font-medium">
					<Menu.Item>
						<Link href="/profile" className="px-2 py-0.5">
							Profile
						</Link>
					</Menu.Item>
					<Menu.Item>
						<Link href="/profile" className="px-2 py-0.5">
							Settings
						</Link>
					</Menu.Item>
					<Menu.Item>
						<Link href="/about" className="px-2 py-0.5">
							About
						</Link>
					</Menu.Item>
				</Menu.Items>
			</Menu>
		</div>
	)
}
