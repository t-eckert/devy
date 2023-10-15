"use client"

import { useEffect, useRef } from "react"

import Cell from "./Cell"

interface Props {
	height: number
	width: number
	cells: Cell[]
}

// Cell dimensions in pixels
const cellWidth = 8
const cellHeight = 16

const gap = 2

export default function Grid({ height, width, cells }: Props) {
	const canvasRef = useRef<HTMLCanvasElement>(
		document.createElement("canvas")
	)

	useEffect(() => {
		const canvas = canvasRef.current
		if (!canvas) return

		const ctx = canvas.getContext("2d")
		if (!ctx) return

		for (const cell of cells) {
			ctx.fillStyle = cell.fill

			const { x, y } = cell

			const col = x * (cellWidth + gap)
			const row = y * (cellHeight + gap)

			ctx.fillRect(col, row, cellWidth, cellHeight)
		}
	}, [canvasRef, cells, height, width])

	return (
		<canvas
			ref={canvasRef}
			width={`${width * cellWidth + gap * (width - 1)}px`}
			height={`${height * cellHeight + gap * (height - 1)}px`}
		/>
	)
}
