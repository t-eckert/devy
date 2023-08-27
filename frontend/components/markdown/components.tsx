interface Props {
	children?: React.ReactNode
}

const components = {
	h1: ({ children }: Props) => (
		<h1 className="text-3xl font-semibold">{children}</h1>
	),
	h2: ({ children }: Props) => (
		<h2 className="mb-1 text-2xl font-semibold">{children}</h2>
	),
	p: ({ children }: Props) => <p>{children}</p>,
	blockquote: ({ children }: Props) => (
		<blockquote className="border-l-4 border-zinc-500 pl-2">
			{children}
		</blockquote>
	),
}

export default components
