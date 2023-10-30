import type { Meta, StoryObj } from "@storybook/react"

import Likes from "./likes"

const meta = {
	title: "Components/Likes",
	component: Likes,
} satisfies Meta<typeof Likes>

export default meta

type Story = StoryObj<typeof Likes>

const onLike = () => {
	console.log("like")
}
const onUnlike = () => {
	console.log("unlike")
}

export const LoggedOutUnliked: Story = {
	args: {
		active: false,
		initialIsLiked: false,
		initialCount: 200,
		onLike,
		onUnlike,
	},
}

export const LoggedInUnliked: Story = {
	args: {
		active: true,
		initialIsLiked: false,
		initialCount: 200,
		onLike,
		onUnlike,
	},
}

export const LoggedInLiked: Story = {
	args: {
		active: true,
		initialIsLiked: true,
		initialCount: 200,
		onLike,
		onUnlike,
	},
}
