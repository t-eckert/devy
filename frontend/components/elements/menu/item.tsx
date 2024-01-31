import * as DropdownMenu from "@radix-ui/react-dropdown-menu"

import Link from "@/components/elements/link"

type SeparatorItem = { type: "separator" }
type LinkItem = {
	type: "link"
	label: string
	tag?: string
	href: string
}
type ButtonItem = {
	type: "button"
	label: string
	tag?: string
	onClick: () => void
}
export type Item = SeparatorItem | LinkItem | ButtonItem

export default function Item({ item }: { item: Item }) {
	switch (item.type) {
		case "separator":
			return (
				<DropdownMenu.Separator className="my-0.5 border-b border-neutral+1 dark:border-neutral-1" />
			)
		case "link":
			return (
				<div className="w-full px-1">
					<DropdownMenu.Item asChild>
						<Link
							href={item.href}
							className={menuItem}
							variant={{ underline: false, styled: false }}
						>
							<span>{item.label}</span>
							{item.tag && (
								<span className={pill}>{item.tag}</span>
							)}
						</Link>
					</DropdownMenu.Item>
				</div>
			)
		case "button":
			return (
				<div className="w-full px-1">
					<DropdownMenu.Item asChild>
						<button className={menuItem} onClick={item.onClick}>
							<span>{item.label}</span>
							{item.tag && (
								<span className={pill}>{item.tag}</span>
							)}
						</button>
					</DropdownMenu.Item>
				</div>
			)
	}
}

const menuItem = [
	"w-full",
	"px-2",
	"py-0.5",

	"outline-none",
	"transition-all",
	"group",
	"text-sm",
	"rounded",
	"flex",
	"flex-row",
	"items-center",
	"justify-between",
	"pointer-cursor",

	"data-[highlighted]:bg-neutral+1",
	"data-[highlighted]:dark:bg-neutral-1",

	"hover:bg-neutral+1",
	"hover:dark:bg-neutral-1",
].join(" ")
const pill = [
	"px-1.5",
	"py-0.5",
	"text-xs",
	"rounded-full",

	"bg-neutral+1",
	"dark:bg-neutral-1",

	"group-hover:bg-neutral+1",
	"group-hover:border-neutral+2",

	"group-hover:dark:bg-neutral-1",
	"group-hover:dark:border-neutral-2",
].join(" ")
