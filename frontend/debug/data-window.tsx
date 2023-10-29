"use client"

import { useRef, useState } from "react"

import Json from "./json"

interface Props {
	name?: string
	defaultOpen?: boolean
	data: any
}

export default function DataWindow({ name, data, defaultOpen }: Props) {
	const ref = useRef<HTMLElement>(null)
	const [showJson, setShowJson] = useState(defaultOpen)

	const drag = (e: React.MouseEvent) => {
		const el = ref.current
		if (!el) return

		const { clientX, clientY } = e
		const { offsetLeft, offsetTop } = el

		const move = (e: MouseEvent) => {
			const { clientX: x, clientY: y } = e
			el.style.left = `${x - clientX + offsetLeft}px`
			el.style.top = `${y - clientY + offsetTop}px`
		}

		el.addEventListener("mousemove", move)
		el.addEventListener("mouseup", () => {
			el.removeEventListener("mousemove", move)
		})
	}

	return (
		<section
			ref={ref}
			className="absolute top-2 right-2 z-50 bg-zinc-800 w-96 rounded-xl border border-zinc-700 shadow-lg"
		>
			<header
				onMouseDown={drag}
				className="p-2 flex flex-row justify-between cursor-pointer select-none"
			>
				<h1 className="text-sm font-medium">{name || "Debugging"}</h1>
				<button
					onClick={() => {
						setShowJson(!showJson)
					}}
					className="text-xs font-mono"
				>
					{showJson ? "Hide" : "Show"}
				</button>
			</header>
			{showJson && (
				<Json
					data={data}
					className="rounded-b-xl border-t border-zinc-700"
				/>
			)}
		</section>
	)
}
