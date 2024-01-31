import type { Meta, StoryObj } from "@storybook/react"

import Card from "./card"

const meta = {
	title: "Elements/Card",
	component: Card,
} satisfies Meta<typeof Card>

export default meta

type Story = StoryObj<typeof Card>

export const Default: Story = {
	args: {
		className: "w-64 px-2 py-4 flex items-center justify-center",
		children: (
			<h1 className="text-neutral-darker dark:text-neutral-lightest">
				Text
			</h1>
		),
	},
}
