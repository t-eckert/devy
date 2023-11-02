import type { Meta, StoryObj } from "@storybook/react"

import Likes from "./likes"

const meta = {
	title: "Components/Likes",
	component: Likes,
} satisfies Meta<typeof Likes>

export default meta

type Story = StoryObj<typeof Likes>

export const LoggedOutUnliked: Story = {
	args: {
		initialCount: 200,
	},
}

export const LoggedInUnliked: Story = {
	args: {
		initialCount: 200,
	},
}

export const LoggedInLiked: Story = {
	args: {
		initialCount: 200,
	},
}
