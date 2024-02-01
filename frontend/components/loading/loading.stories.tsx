import type { Meta, StoryObj } from "@storybook/react"

import Loading from "./loading"

const meta = {
	title: "Loading",
	component: Loading,
} satisfies Meta<typeof Loading>

export default meta

type Story = StoryObj<typeof Loading>

export const Main: Story = {
	args: {},
	decorators: [
		(Story) => (
			<div className="w-[80vw] h-[80vh] bg-zinc-100">
				<Story />
			</div>
		),
	],
}
