import type { Meta, StoryObj } from "@storybook/react"

import Collection from "./collection"

const meta = {
	title: "Post/Collection",
	component: Collection,
} satisfies Meta<typeof Collection>

export default meta

type Story = StoryObj<typeof Collection>

export const Default: Story = {
	args: {
		posts: [
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
		],
	},
}
