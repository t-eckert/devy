import type { Meta, StoryObj } from "@storybook/react"

import MemberCard from "./member-card"

const meta = {
	title: "Profile/MemberCard",
	component: MemberCard,
} satisfies Meta<typeof MemberCard>

export default meta

type Story = StoryObj<typeof MemberCard>

export const NewUser: Story = {
	args: {
		user: {
			username: "johndoe",
			email: "johndoe@email.com",
			githubUsername: "johndoe",
			createdAt: "2021-08-01T00:00:00.000Z",
			updatedAt: "2021-08-01T00:00:00.000Z",
			role: "user",
			status: "active",
		},
		profile: {
			id: "9ea9cf38-e3d3-4663-accb-fa4ab117feec",
			userId: "9ea9cf38-e3d3-4663-accb-fa4ab117feec",
			avatarUrl: "https://avatars.githubusercontent.com/u/10047061?v=4",
			displayName: "John Doe",
			bio: "I am a software engineer.",
			website: "https://johndoe.com",
			createdAt: "2021-08-01T00:00:00.000Z",
			updatedAt: "2021-08-01T00:00:00.000Z",
		},
		blogs: [],
	},
}

export const LongName: Story = {
	args: {
		user: {
			username: "johndoe",
			email: "johndoe@email.com",
			githubUsername: "johndoe",
			createdAt: "2021-08-01T00:00:00.000Z",
			updatedAt: "2021-08-01T00:00:00.000Z",
			role: "user",
			status: "active",
		},
		profile: {
			id: "9ea9cf38-e3d3-4663-accb-fa4ab117feec",
			userId: "9ea9cf38-e3d3-4663-accb-fa4ab117feec",
			avatarUrl:
				"https://images.unsplash.com/photo-1496345875659-11f7dd282d1d?q=80&w=1470&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
			displayName: "Wilhelm Friedrich von Rosenbaum-Müller",
			bio: "I am a software engineer.",
			website: "https://johndoe.com",
			createdAt: "2021-08-01T00:00:00.000Z",
			updatedAt: "2021-08-01T00:00:00.000Z",
		},
		blogs: [],
	},
}
