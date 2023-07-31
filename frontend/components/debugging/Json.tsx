"use client"

import { useRef, useState } from "react"

interface Props {
	data: any
}

export default function Json({ data }: Props) {
	return (
		<section className="bg-zinc-800 max-w-xl rounded-xl border border-zinc-700">
			<pre className="p-2 text-sm overflow-scroll border-t border-zinc-700">
				<code>{JSON.stringify(data, null, 2)}</code>
			</pre>
		</section>
	)
}
