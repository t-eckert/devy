import type { Meta, StoryObj } from "@storybook/react"

import Card from "./card"

const meta = {
	title: "Components/Card",
	component: Card,
} satisfies Meta<typeof Card>

export default meta

type Story = StoryObj<typeof Card>

export const Default: Story = {
	args: {
		children: (
			<h1 className="text-neutral-darker dark:text-neutral-lightest">
				Title
			</h1>
		),
	},
}
