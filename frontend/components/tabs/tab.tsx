import { cva, type VariantProps } from "cva"
import { SetStateAction } from "react"

interface Props extends VariantProps<typeof styles> {
	children: React.ReactNode
	id: string
	label: string
	isSelected: boolean
	setSelected: SetStateAction<any>
}

export default function Tab({ children, id, label, isSelected, setSelected }: Props) {
	return (
		<button
			className={styles({ isSelected })}
			onClick={() => setSelected(id)}
			aria-label={`Select feed ${label}`}
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
		"font-medium",

		"bg-neutral+3",
		"dark:bg-neutral-2",

		"md:hover:pl-4",
		"md:focus:pl-3",
		"md:hover:bg-neutral-1",
		"md:hover:text-neutral+1",

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
				true: [
					"border",

					"text-neutral-3",
					"bg-neutral+2",
					"border-neutral+1",

					"dark:text-neutral+3",
					"dark:bg-neutral-1",
					"dark:border-neutral-1",

					"focus:shadow",
				],
				false: [
					"border",
					"text-neutral",
					"border-neutral+3",
					"dark:text-neutral",
					"dark:border-neutral-2",
				],
			},
		},
		defaultVariants: {
			isSelected: false,
		},
	}
)
