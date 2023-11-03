import { cva, type VariantProps } from "cva"
import { SetStateAction } from "react"

interface Props extends VariantProps<typeof styles> {
	children: React.ReactNode
	id: string
	isSelected: boolean
	setSelected: SetStateAction<any>
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
		"md:hover:pl-3",
		"md:focus:pl-3",
		"flex",
		"flex-row",
		"justify-center",
		"md:justify-start",
		"items-center",
		"rounded-md",
		"transition-all",
		"focus:outline-none",
		"focus:ring-2",
		"focus:ring-zinc-200",
	],
	{
		variants: {
			isSelected: {
				true: "font-medium border border-neutral-medium shadow",
				false: "",
			},
		},
		defaultVariants: {
			isSelected: false,
		},
	}
)
