import type { Meta, StoryObj } from "@storybook/react"

import Input from "./input"

const meta = {
	title: "Elements/Input",
	component: Input,
} satisfies Meta<typeof Input>

export default meta

type Story = StoryObj<typeof Input>

export const Text: Story = {
	args: {
		placeholder: "Placeholder text",
	},
}
