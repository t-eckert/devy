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
		"border",

		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-700",
		"focus:ring-offset-1",
		"dark:focus:ring-zinc-200",
		"dark:text-zinc-50",

		"disabled:cursor-not-allowed",

		"transition-all",
	],
	{
		variants: {
			intent: {
				primary: [
					"px-2.5",
					"py-0.5",

					"border-neutral-3",
					"bg-neutral-2",
					"text-neutral+2",

					"dark:border-neutral+2",
					"dark:bg-neutral+2",
					"dark:text-neutral-2",

					"disabled:bg-neutral+1",
					"disabled:border-neutral+1",
					"disabled:text-neutral",
				],
				secondary: [
					"px-2.5",
					"py-0.5",
					"border-zinc-100",
					"bg-zinc-100",
					"text-zinc-950",
					"border-zinc-200",
					"hover:bg-zinc-200",
					"dark:border-neutral-medium",
					"dark:bg-neutral-medium",
					"dark:text-neutral-light",
				],
				tertiary: ["px-1", "py-0.5", "border-none"],
				creative: [],
				destructive: [],
				cautionary: [],
			},
		},
		defaultVariants: {
			intent: "primary",
		},
	}
)
