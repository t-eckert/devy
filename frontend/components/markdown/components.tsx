interface Props {
	children?: React.ReactNode
	src?: string
	alt?: string
}

const components = {
	h1: ({ children }: Props) => (
		<h1 className="mb-4 text-6xl font-semibold text-neutral-1 dark:text-neutral+1">
			{children}
		</h1>
	),
	h2: ({ children }: Props) => (
		<h2 className="mx-auto w-full max-w-2xl mt-4 mb-4 text-4xl font-semibold">
			{children}
		</h2>
	),
	h3: ({ children }: Props) => (
		<h3 className="mx-auto w-full max-w-2xl mt-2 mb-2 text-2xl font-semibold">
			{children}
		</h3>
	),
	h4: ({ children }: Props) => (
		<h4 className="mx-auto w-full max-w-2xl mb-1 text-xl font-semibold">
			{children}
		</h4>
	),
	h5: ({ children }: Props) => (
		<h5 className="mx-auto w-full max-w-2xl mb-1 text-lg font-semibold">
			{children}
		</h5>
	),
	h6: ({ children }: Props) => (
		<h6 className="mx-auto w-full max-w-2xl mb-1 font-semibold">
			{children}
		</h6>
	),
	p: ({ children }: Props) => (
		<p className="mx-auto w-full max-w-2xl mb-4 leading-snug">{children}</p>
	),
	blockquote: ({ children }: Props) => (
		<blockquote className="border-l border-neutral-1 pl-2">
			{children}
		</blockquote>
	),
	pre: ({ children }: Props) => (
		<pre className="mx-auto w-full max-w-2xl mb-4 mt-2 bg-neutral-3 font-mono px-2 py-1 rounded text-neutral+1 bg-zinc-800 text-sm">
			{children}
		</pre>
	),
	code: ({ children }: Props) => (
		<code className="font-mono">{children}</code>
	),
	ul: ({ children }: Props) => (
		<ul className="mx-auto w-full max-w-2xl mb-4 list-disc list-inside">
			{children}
		</ul>
	),
	li: ({ children }: Props) => (
		<li className="mb-1 leading-snug">{children}</li>
	),
	ol: ({ children }: Props) => (
		<ol className="mx-auto w-full max-w-2xl mb-4 list-decimal list-inside">
			{children}
		</ol>
	),
	table: ({ children }: Props) => (
		<table className="table-auto mt-2 mb-4 w-full">{children}</table>
	),
	thead: ({ children }: Props) => (
		<thead className="border-b border-b-neutral text-neutral-1 dark:text-neutral+1">
			{children}
		</thead>
	),
	th: ({ children }: Props) => (
		<th className="px-1 py-0.5 font-semibold text-left">{children}</th>
	),
	td: ({ children }: Props) => (
		<td className="px-1 py-0.5 border-b border-b-neutral+1">{children}</td>
	),
	img: ({ src, alt }: Props) => (
		<img
			src={src}
			alt={alt}
			className="mx-auto my-4 max-w-2xl rounded shadow-lg bg-neutral+1 flex text-center"
		/>
	),
}

export default components
