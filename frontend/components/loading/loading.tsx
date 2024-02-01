"use client"

import { useEffect, useRef, useState } from "react"

export type Cell = [x: number, y: number]

const cellWidth = 8
const cellHeight = 16
const gap = 2
const width = 50
const height = 25
const fill = "#71717a"
const seedDensity = 0.15
const interval = 100

export default function Loading() {
  const canvasRef = useRef<HTMLCanvasElement>(document.createElement("canvas"))

  const [cells, setCells] = useState(generateCells())

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    const redraw = () => {
      drawCells(ctx, cells)
      setCells(step(cells))
    }

    const intervalId = setInterval(redraw, interval)

    return () => clearInterval(intervalId)
  }, [canvasRef, cells, height, width])

  return (
    <div
      className="w-full h-full flex items-center justify-center rounded-xl overflow-clip motion-safe:animate-pulse"
      onClick={() => setCells(generateCells())}
    >
      <span className="motion-safe:hidden">Loading...</span>
      <canvas
        ref={canvasRef}
        width={`${width * cellWidth + gap * (width - 1)}px`}
        height={`${height * cellHeight + gap * (height - 1)}px`}
        className="motion-reduce:hidden"
      />
    </div>
  )
}

function drawCells(ctx: CanvasRenderingContext2D, cells: Cell[]) {
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
  for (const cell of cells) {
    ctx.fillStyle = fill

    const [x, y] = cell

    const col = x * (cellWidth + gap)
    const row = y * (cellHeight + gap)

    ctx.fillRect(col, row, cellWidth, cellHeight)
  }
}

function generateCells(): Cell[] {
  const cells: Cell[] = []

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (Math.random() > seedDensity) continue
      cells.push([x, y])
    }
  }

  return cells
}

function step(cells: Cell[]): Cell[] {
  const next: Cell[] = []

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (cellDoesLive(cells, x, y)) {
        next.push([x, y])
      }
    }
  }

  return next
}

function isLiving(cells: Cell[], x: number, y: number): boolean {
  return cells.some(([cx, cy]) => cx === x && cy === y)
}

export function getLiveNeighbors(cells: Cell[], x: number, y: number): number {
  let count = 0

  for (let dy = -1; dy <= 1; dy++) {
    for (let dx = -1; dx <= 1; dx++) {
      if (dx === 0 && dy === 0) continue

      const nx = x + dx
      const ny = y + dy

      if (cells.some(([cx, cy]) => cx === nx && cy === ny)) {
        count++
      }
    }
  }

  return count
}

// Rules
// Any live cell with fewer than two live neighbors dies, as if by underpopulation.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
export function cellDoesLive(cells: Cell[], x: number, y: number): boolean {
  const liveNeighbors = getLiveNeighbors(cells, x, y)
  if (isLiving(cells, x, y)) {
    if (liveNeighbors === 2 || liveNeighbors === 3) {
      return true
    }

    return false
  }

  return liveNeighbors === 3
}
