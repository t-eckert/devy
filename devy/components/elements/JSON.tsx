interface Props {
	raw: any
}

export default function JSONComponent({ raw }: Props) {
	return (
		<pre>
			<code>{JSON.stringify(raw, null, 2)}</code>
		</pre>
	)
}
