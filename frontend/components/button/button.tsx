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
		"pointer-cursor",
		"transition-all",
		"rounded-lg",
		"text-sm",
		"font-medium",
		"px-2.5",
		"py-0.5",
		"border",
		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-700",
		"focus:ring-offset-1",
		"dark:focus:ring-zinc-200",
		"dark:text-zinc-50",
	],
	{
		variants: {
			intent: {
				primary: [
					"border-neutral-darkest",
					"bg-neutral-darker",
					"text-neutral-lighter",
					"dark:border-neutral-lightest",
					"dark:bg-neutral-lightest",
					"dark:text-neutral-darker",
				],
				secondary: [
					"border-zinc-100",
					"bg-zinc-100",
					"text-zinc-950",
					"border-zinc-200",
					"hover:bg-zinc-200",
					"dark:border-neutral-medium",
					"dark:bg-neutral-medium",
					"dark:text-neutral-light",
				],
				tertiary: [],
				creative: [],
				destructive: [],
				cautionary: [],
			},
		},
	}
)
