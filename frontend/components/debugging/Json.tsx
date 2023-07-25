interface Props {
	data: any
}

export default function Json({ data }: Props) {
	return (
		<pre className="p-2 rounded-xl bg-zinc-800 text-white text-xs overflow-scroll">
			<code>{JSON.stringify(data, null, 2)}</code>
		</pre>
	)
}
