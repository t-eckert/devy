import Grid, { Cell } from "@/components/grid"

const red = "#DC2626"

// Cells that spell out 404
const cells: Cell[] = [
	// 4
	[0, 0],
	[0, 1],
	[0, 2],
	[1, 0],
	[1, 1],
	[1, 2],
	[2, 2],
	[3, 0],
	[3, 1],
	[3, 2],
	[4, 0],
	[4, 1],
	[4, 2],
	[3, 3],
	[3, 4],
	[4, 3],
	[4, 4],
	// 0
	[6, 0],
	[6, 1],
	[6, 2],
	[6, 3],
	[6, 4],
	[7, 0],
	[7, 1],
	[7, 2],
	[7, 3],
	[7, 4],
	[8, 0],
	[8, 4],
	[9, 0],
	[9, 1],
	[9, 2],
	[9, 3],
	[9, 4],
	[10, 0],
	[10, 1],
	[10, 2],
	[10, 3],
	[10, 4],
	// 4
	[12, 0],
	[12, 1],
	[12, 2],
	[13, 0],
	[13, 1],
	[13, 2],
	[14, 2],
	[15, 0],
	[15, 1],
	[15, 2],
	[16, 0],
	[16, 1],
	[16, 2],
	[15, 3],
	[15, 4],
	[16, 3],
	[16, 4],
].map(([x, y]) => ({ x, y, fill: red }))

export default function Component404() {
	return (
		<div className="flex flex-col items-center gap-3">
			<Grid height={5} width={17} cells={cells} />
			<span className="sr-only">404</span>
		</div>
	)
}
