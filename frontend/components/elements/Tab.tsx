import { cva, type VariantProps } from "cva"

interface Props extends VariantProps<typeof styles> {
	children: React.ReactNode
	id: string
	isSelected: boolean
	setSelected: (id: string) => void
}

export default function Tab({ children, id, isSelected, setSelected }: Props) {
	return (
		<button
			className={styles({ isSelected })}
			onClick={() => setSelected(id)}
		>
			{children}
		</button>
	)
}

const styles = cva(
	[
		"px-2",
		"py-0.5",
		"text-sm",
		"sm:hover:pl-3",
		"sm:focus:pl-3",
		"transition-all",
		"sm:w-44",
		"flex",
		"justify-center",
		"sm:items-center",
		"focus:outline-none",
		"focus:ring-2",
		"focus:ring-zinc-200",
		"rounded-md",
	],
	{
		variants: {
			isSelected: {
				true: "text-white font-medium bg-zinc-700 border border-zinc-700 shadow",
				false: "text-zinc-500 border border-transparent",
			},
		},
		defaultVariants: {
			isSelected: false,
		},
	}
)
