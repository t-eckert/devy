import { VariantProps, cva } from "cva"
import { ButtonHTMLAttributes } from "react"

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
	variant?: VariantProps<typeof styles>
}

export default function Button({ children, variant, ...props }: Props) {
	return (
		<button className={styles(variant)} {...props}>
			{children}
		</button>
	)
}

const styles = cva(
	[
		"px-2",
		"pointer-cursor",
		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-700",
		"focus:ring-offset-1",
		"dark:focus:ring-zinc-200",
		"rounded-md",
		"dark:text-zinc-50",
	],
	{
		variants: {
			intent: {
				primary: [
					"border",
					"dark:border-zinc-200",
					"dark:bg-zinc-200",
					"dark:text-zinc-950",
				],
			},
		},
	}
)
