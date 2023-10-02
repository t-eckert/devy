"use client"

import { useEffect, useRef, useState } from "react"

function LoadingGOL() {
	const cols = 40
	const rows = 40
	const gap = 2

	const [board, setBoard] = useState(generateRandomPattern(cols, rows))

	const canvasRef = useRef<HTMLCanvasElement>(
		document.createElement("canvas")
	)

	useEffect(() => {
		const canvas = canvasRef.current
		const context = canvas.getContext("2d")

		if (!context) return

		context.fillStyle = "#fbfbfb"

		const rectSize = (canvas.width - gap * (cols + 1)) / cols

		// Draw
		for (let col = 0; col < cols; col++) {
			for (let row = 0; row < rows; row++) {
				const alive = board.at(col)?.at(row) ?? false
				if (alive) {
					const x = col * (rectSize + gap) + gap
					const y = row * (rectSize + gap) + gap

					context.fillRect(x, y, rectSize, rectSize)
				}
			}
		}
	}, [board])

	return (
		<div>
			<canvas
				ref={canvasRef}
				width={`${(cols + gap) * 10}px`}
				height={`${(rows + gap) * 10}px`}
			/>
		</div>
	)
}

function LoadingStatic() {
	return <div>Loading...</div>
}

export default function Loading() {
	const prefersReducedMotion = window.matchMedia(
		"(prefers-reduced-motion: reduce)"
	).matches

	if (!prefersReducedMotion) return <LoadingGOL />

	return <LoadingStatic />
}

// Game of Life!

type Board = Cell[][]
type Cell = boolean

// The higher this number, the less likely it is that an initially generated cell is alive.
// Use this to tune how many cells are alive at the beginning of the game.
const lifeThreshold = 0.9

const generateRandomPattern = (cols: number, rows: number): Board => {
	let col
	let row
	let board: Board = []

	for (col = 0; col < cols; col++) {
		const column: Cell[] = []
		for (row = 0; row < rows; row++) {
			column.push(Math.random() > lifeThreshold)
		}
		board.push(column)
	}

	return board
}

const next = (board: Board): Board => {
	return board
}
