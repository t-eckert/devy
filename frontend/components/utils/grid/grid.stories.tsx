import type { Meta, StoryObj } from "@storybook/react"

import Grid from "./grid"
import { d } from "@tanstack/react-query-devtools/build/legacy/devtools-c71c5f06"

const meta = {
	title: "Utils/Grid",
	component: Grid,
} satisfies Meta<typeof Grid>

export default meta

type Story = StoryObj<typeof Grid>

const red = "#ff0000"

const greens = [
	"#dcfce7",
	"#bbf7d0",
	"#86efac",
	"#4ade80",
	"#22c55e",
	"#16a34a",
	"#15803d",
	"#166534",
	"#14532d",
]

export const Main: Story = {
	args: {
		height: 30,
		width: 30,
		cells: [{ x: 0, y: 0, fill: red }],
	},
}

export const Filled: Story = {
	args: {
		height: greens.length,
		width: greens.length,
		cells: Array.from(
			{ length: greens.length * greens.length },
			(_, i) => ({
				x: i % greens.length,
				y: Math.floor(i / greens.length),
				fill: greens[
					(i % greens.length) * greens.length +
						Math.floor(i / greens.length)
				],
			})
		),
	},
}

const isPrime = (n: number) => {
	if (n < 2) return false
	if (n === 2) return true
	if (n % 2 === 0) return false
	for (let i = 3; i * i <= n; i += 2) {
		if (n % i === 0) return false
	}
	return true
}

const height = 40
const width = 80
export const Primes: Story = {
	args: {
		height,
		width,
		cells: Array.from({ length: height * width }, (_, i) => ({
			x: i % width,
			y: Math.floor(i / width),
			fill: red,
		})).filter((_, i) => isPrime(i)),
	},
}
