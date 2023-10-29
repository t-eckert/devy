import type { Meta, StoryObj } from "@storybook/react"

import Like from "./like"

const meta = {
	title: "Elements/Like",
	component: Like,
	parameters: {
		layout: "centered",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof Like>

export default meta

type Story = StoryObj<typeof Like>

export const Primary: Story = {
	args: {
		active: false,
		initialIsLiked: false,
		initialCount: 200,
		onLike: () => {
			console.log("like")
		},
		onUnlike: () => {
			console.log("unlike")
		},
	},
}
