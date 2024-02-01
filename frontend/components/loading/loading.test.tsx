import { expect, test } from "vitest"
import { getLiveNeighbors, cellDoesLive } from "./loading"
import { type Cell } from "./loading"

test("getLiveNeighbors calculates the number of living neighbors", () => {
	interface Case {
		given: {
			cells: Cell[]
			x: number
			y: number
		}
		expected: number
	}

	const cases: Case[] = [
		// Cell has 3 live neighbors
		{
			given: {
				cells: [
					[0, 0],
					[1, 0],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: 3,
		},
		// Cell has 2 live neighbors
		{
			given: {
				cells: [
					[0, 0],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: 2,
		},
		// Cell has 1 live neighbors
		{
			given: {
				cells: [[0, 0]],
				x: 1,
				y: 1,
			},
			expected: 1,
		},
		// Cell has 4 live neighbors
		{
			given: {
				cells: [
					[0, 0],
					[0, 1],
					[1, 0],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: 4,
		},
	]

	for (const { given, expected } of cases) {
		const actual = getLiveNeighbors(given.cells, given.x, given.y)
		expect(actual).toBe(expected)
	}
})

test("cellDoesLive determines if a cell should live or die", () => {
	interface Case {
		given: {
			cells: Cell[]
			x: number
			y: number
		}
		expected: boolean
	}

	const cases: Case[] = [
		// A live cell with fewer than two live neighbors dies
		{
			given: {
				cells: [
					[1, 1],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: false,
		},
		// A live cell with two live neighbors lives on to the next generation
		{
			given: {
				cells: [
					[0, 0],
					[1, 0],
					[2, 0],
				],
				x: 1,
				y: 0,
			},
			expected: true,
		},
		// A live cell with three live neighbors lives on to the next generation
		{
			given: {
				cells: [
					[0, 0],
					[1, 1],
					[1, 0],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: true,
		},
		// A dead cell with exactly three live neighbors becomes a live cell
		{
			given: {
				cells: [
					[0, 0],
					[1, 0],
					[2, 0],
				],
				x: 1,
				y: 1,
			},
			expected: true,
		},
	]

	for (const { given, expected } of cases) {
		const actual = cellDoesLive(given.cells, given.x, given.y)
		expect(actual).toBe(expected)
	}
})
