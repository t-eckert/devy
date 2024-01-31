import type { Meta, StoryObj } from "@storybook/react"

import { useEffect, useState } from "react"

import LikesLoggedIn from "./likes-logged-in"
import LikesLoggedOut from "./likes-logged-out"

interface Props {
	hasUser: boolean
	count: number
	initialLiked: boolean
}

function Likes({ hasUser, count, initialLiked }: Props) {
	const [isLiked, setIsLiked] = useState(initialLiked)

	const onClick = () => {
		setIsLiked(!isLiked)
	}

	if (hasUser) {
		return (
			<LikesLoggedIn
				count={count}
				title="Article"
				isLiked={isLiked}
				onClick={onClick}
			/>
		)
	}
	return <LikesLoggedOut count={count} />
}

const meta = {
	title: "Post/Likes",
	component: Likes,
} satisfies Meta<typeof Likes>

export default meta

type Story = StoryObj<typeof Likes>

export const LoggedOutUnliked: Story = {
	args: {
		hasUser: false,
		count: 20,
		initialLiked: false,
	},
}

export const LoggedInUnliked: Story = {
	args: {
		hasUser: true,
		count: 20,
		initialLiked: false,
	},
}

export const LoggedInLiked: Story = {
	args: {
		hasUser: true,
		count: 20,
		initialLiked: true,
	},
}
