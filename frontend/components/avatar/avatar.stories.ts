import type { Meta, StoryObj } from "@storybook/react"

import Avatar from "./avatar"

const meta = {
	title: "Components/Avatar",
	component: Avatar,
	parameters: {
		layout: "centered",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof Avatar>

export default meta

type Story = StoryObj<typeof Avatar>

export const Initials: Story = {
	args: {
		name: "John Doe",
	},
}

export const Image: Story = {
	args: {
		name: "John Doe",
		avatarUrl:
			"https://images.unsplash.com/photo-1438761681033-6461ffad8d80?auto=format&fit=crop&q=80&w=2670&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
	},
}

export const SingleInitial: Story = {
	args: {
		name: "Johnny",
	},
}

export const TripleInitials: Story = {
	args: {
		name: "John Doe Smith",
	},
}
