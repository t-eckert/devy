"use client"

// This component is based on RadixUI's DropdownMenu component.
// https://www.radix-ui.com/primitives/docs/components/dropdown-menu

import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { VariantProps, cva } from "cva"

import Link from "@/components/link"

type Relation = "below-aligned-left"

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

interface Props {
	icon: React.ReactNode
	variant: VariantProps<typeof content>
	ariaLabel: string
	relation: Relation
	items: Item[]
	initialIsOpen?: boolean
}

export default function Menu({
	icon,
	variant,
	ariaLabel,
	items,
	initialIsOpen,
}: Props) {
	return (
		<DropdownMenu.Root defaultOpen={initialIsOpen}>
			<DropdownMenu.Trigger asChild>
				<button className={trigger} aria-label={ariaLabel}>
					{icon}
				</button>
			</DropdownMenu.Trigger>

			<DropdownMenu.Portal>
				<DropdownMenu.Content
					className={content(variant)}
					sideOffset={5}
					align="end"
				>
					{items.map((item, index) => (
						<Item key={index} item={item} />
					))}
				</DropdownMenu.Content>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	)
}

const trigger = [
	"text-neutral-2",
	"dark:text-neutral+2",
	"inline-flex",
	"items-center",
	"rounded",
	"p-1",

	"hover:bg-neutral+1",
	"hover:dark:bg-neutral-1",

	"data-[state=open]:bg-neutral+3",
	"data-[state=open]:text-neutral-3",
	"data-[state=open]:ring-1",
	"data-[state=open]:ring-neutral+1",

	"data-[state=open]:dark:bg-neutral-3",
	"data-[state=open]:dark:text-neutral+2",
	"data-[state=open]:dark:ring-1",
	"data-[state=open]:dark:ring-neutral-1",

	"data-[state=open]:shadow",
].join(" ")
const content = cva(
	[
		"rounded-md",
		"border",

		"border-neutral+1",
		"text-neutral-2",
		"bg-neutral+3",

		"dark:border-neutral-1",
		"dark:text-neutral+2",
		"dark:bg-neutral-3",

		"data-[side=top]:animate-slideDownAndFade",
		"data-[side=right]:animate-slideLeftAndFade",
		"data-[side=bottom]:animate-slideUpAndFade",
		"data-[side=left]:animate-slideRightAndFade",
	],
	{
		variants: {
			hug: {
				true: ["w-min"],
				false: ["w-48"],
			},
		},
		defaultVariants: {
			hug: false,
		},
	}
)

const Item = ({ item }: { item: Item }) => {
	switch (item.type) {
		case "separator":
			return (
				<DropdownMenu.Separator className="my-1 border-b border-neutral+1 dark:border-neutral-1" />
			)
		case "link":
			return (
				<DropdownMenu.Item asChild>
					<Link
						href={item.href}
						className={menuItem}
						variant={{ underline: false, styled: false }}
					>
						<span>{item.label}</span>
						{item.tag && <span className={pill}>{item.tag}</span>}
					</Link>
				</DropdownMenu.Item>
			)
		case "button":
			return (
				<DropdownMenu.Item asChild>
					<button className={menuItem} onClick={item.onClick}>
						<span>{item.label}</span>
						{item.tag && <span className={pill}>{item.tag}</span>}
					</button>
				</DropdownMenu.Item>
			)
	}
}

const menuItem = [
	"mx-1",
	"my-1",
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
	"border",

	"border-neutral+1",
	"bg-neutral+1",

	"dark:border-neutral-1",
	"dark:bg-neutral-1",

	"group-hover:bg-neutral+1",
	"group-hover:border-neutral+2",

	"group-hover:dark:bg-neutral-1",
	"group-hover:dark:border-neutral-2",
].join(" ")
