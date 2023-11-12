import type { Meta, StoryObj } from "@storybook/react"

import MenuSample from "./sample-menu"

const meta = {
	title: "Components/MenuSample",
	component: MenuSample,
} satisfies Meta<typeof MenuSample>

export default meta

type Story = StoryObj<typeof MenuSample>

export const Sample: Story = {
	args: {},
}
