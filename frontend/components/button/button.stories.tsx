import type { Meta, StoryObj } from "@storybook/react"

import Button from "./button"

const meta = {
	title: "Components/Button",
	component: Button,
} satisfies Meta<typeof Button>

export default meta

type Story = StoryObj<typeof Button>

export const Primary: Story = {
	args: {
		children: "Primary",
		variant: {
			intent: "primary",
		},
	},
}

export const Secondary: Story = {
	args: {
		children: "Secondary",
		variant: {
			intent: "secondary",
		},
	},
}