import type { Meta, StoryObj } from "@storybook/react"

import Link from "./link"

const meta = {
	title: "Elements/Link",
	component: Link,
	parameters: {
		layout: "centered",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof Link>

export default meta

type Story = StoryObj<typeof Link>

export const NoUnderline: Story = {
	args: {
		href: "#",
		children: "No underline",
		variant: {
			underline: false,
		},
	},
}

export const Underline: Story = {
	args: {
		href: "#",
		children: "No underline",
		variant: {
			underline: true,
		},
	},
}