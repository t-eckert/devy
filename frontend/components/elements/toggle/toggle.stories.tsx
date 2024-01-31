import type { Meta, StoryObj } from "@storybook/react"

import Toggle from "./toggle"

const meta = {
	title: "Elements/Toggle",
	component: Toggle,
} satisfies Meta<typeof Toggle>

export default meta

type Story = StoryObj<typeof Toggle>

export const ToggleDefaultOff: Story = {
	args: {
		id: "toggle-default-off",
	},
}

export const ToggleDefaultOn: Story = {
	args: {
		id: "toggle-default-on",
		defaultChecked: true,
	},
}

export const ToggleDisabled: Story = {
	args: {
		id: "toggle-disabled",
		disabled: true,
	},
}

export const ToggleRequired: Story = {
	args: {
		id: "toggle-required",
		required: true,
	},
}
