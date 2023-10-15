"use client"

interface Props {
	data: any
	className?: string
}

export default function Json({ data, className }: Props) {
	return (
		<pre
			className={[
				"p-2 text-sm max-w-xl overflow-scroll bg-zinc-800",
				className,
			].join(" ")}
		>
			<code>{JSON.stringify(data, null, 2)}</code>
		</pre>
	)
}
