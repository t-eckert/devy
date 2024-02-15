import type { Meta, StoryObj } from "@storybook/react"

import Header from "./header"

const meta = {
	title: "Post/Header",
	component: Header,
} satisfies Meta<typeof Header>

export default meta

type Story = StoryObj<typeof Header>

export const Default: Story = {
	args: {
		post:
		{
			id: "4c9b32bb-5488-46c3-8cdf-87ca2617b945",
			slug: "hello-world",

			blogSlug: "my-blog",
			blogName: "My Blog",
			authorUsername: "t-eckert",
			authorName: "Tobias Eckert",

			title: "Hello, world!",
			content: "This is my first post!",
			tags: ["hello", "world"],
			createdAt: "2021-08-01T12:00:00Z",
			updatedAt: "2021-08-01T12:00:00Z",
			likes: 20,
		},
	},
}