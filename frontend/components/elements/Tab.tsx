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
		"hover:pl-3",
		"transition-all",
		"w-44",
		"flex",
		"justify-between",
		"items-center",
	],
	{
		variants: {
			isSelected: {
				true: "text-white font-medium border border-zinc-700 shadow rounded-md",
				false: "text-zinc-100 border border-transparent",
			},
		},
		defaultVariants: {
			isSelected: false,
		},
	}
)
