interface Props {
	children?: React.ReactNode
}

const components = {
	h1: ({ children }: Props) => (
		<h1 className="mb-2 text-3xl font-semibold">{children}</h1>
	),
	h2: ({ children }: Props) => (
		<h2 className="mb-1 text-2xl font-semibold">{children}</h2>
	),
	p: ({ children }: Props) => <p className="mb-2">{children}</p>,
	blockquote: ({ children }: Props) => (
		<blockquote className="border-l-4 border-zinc-500 pl-2">
			{children}
		</blockquote>
	),
	code: ({ children }: Props) => (
		<code className="font-mono px-2 py-1 rounded bg-zinc-800">
			{children}
		</code>
	),
	ul: ({ children }: Props) => (
		<ul className="mb-2 list-disc list-inside">{children}</ul>
	),
}

export default components
