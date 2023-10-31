import type { Meta, StoryObj } from "@storybook/react"

import Markdown from "./markdown"

const meta = {
	title: "Components/Markdown",
	component: Markdown,
} satisfies Meta<typeof Markdown>

export default meta

type Story = StoryObj<typeof Markdown>

export const Default: Story = {
	args: {
		content: "# Hello world",
	},
}
