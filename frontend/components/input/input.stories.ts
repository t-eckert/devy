import type { Meta, StoryObj } from "@storybook/react"

import Input from "./Input"

const meta = {
	title: "Elements/Input",
	component: Input,
	parameters: {
		layout: "centered",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof Input>

export default meta

type Story = StoryObj<typeof Input>

export const Text: Story = {
	args: {
		placeholder: "Placeholder text",
	},
}
