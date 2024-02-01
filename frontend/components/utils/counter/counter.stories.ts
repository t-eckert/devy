import type { Meta, StoryObj } from "@storybook/react"

import Counter from "./counter"

const meta = {
	title: "Utils/Counter",
	component: Counter,
} satisfies Meta<typeof Counter>

export default meta

type Story = StoryObj<typeof Counter>

export const Hundreds: Story = {
	args: {
		count: 100,
	},
}

export const Thousands: Story = {
	args: {
		count: 2900,
	},
}

export const TensOfThousands: Story = {
	args: {
		count: 30390,
	},
}

export const HundredsOfThousands: Story = {
	args: {
		count: 230390,
	},
}

export const Millions: Story = {
	args: {
		count: 23000930,
	},
}
