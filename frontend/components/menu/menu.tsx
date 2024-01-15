"use client"

// This component is based on RadixUI's DropdownMenu component.
// https://www.radix-ui.com/primitives/docs/components/dropdown-menu

import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { VariantProps, cva } from "cva"

import ItemComponent, { type Item } from "./item"

type Relation = "below-aligned-left"

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
						<ItemComponent key={index} item={item} />
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

	"focus:outline-none",
	"focus:ring-1",
	"focus:ring-neutral-1",
	"focus:dark:ring-neutral+1",

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
		"py-1.5",
		"rounded-md",
		"border",
		"flex",
		"flex-col",
		"gap-1",
		"backdrop-blur-lg",

		"border-neutral+1",
		"text-neutral-2",
		"bg-neutral+3/70",

		"dark:border-neutral-1",
		"dark:text-neutral+2",
		"dark:bg-neutral-3/70",

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
