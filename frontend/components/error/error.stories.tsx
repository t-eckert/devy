import type { Meta, StoryObj } from "@storybook/react"

import Error from "./error"

const meta = {
	title: "Error",
	component: Error,
} satisfies Meta<typeof Error>

export default meta

type Story = StoryObj<typeof Error>

export const NotFoundError: Story = {
	args: {
		error: {
			name: "NotFoundError",
			message: "Unable to find the requested resource",
		},
	},
}

export const ServerError: Story = {
	args: {
		error: {
			name: "ServerError",
			message: "Server did not respond correctly",
		},
	},
}
