import { VariantProps, cva } from "cva"
import { ButtonHTMLAttributes } from "react"

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
	variant?: VariantProps<typeof styles>
}

export default function Button({ children, variant, className, ...props }: Props) {
	return (
		<button className={[styles(variant), className].join(" ")} {...props}>
			{children}
		</button>
	)
}

const styles = cva(
	[
		"pointer-cursor",
		"transition-all",
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
					"px-2.5",
					"rounded-md",
					"font-medium",
					"border",
					"border-zinc-700",
					"bg-zinc-700",
					"text-zinc-50",
					"dark:border-zinc-200",
					"dark:bg-zinc-200",
					"dark:text-zinc-950",
				],
				secondary: [
					"px-2.5",
					"rounded-md",
					"font-medium",
					"border",
					"border-zinc-100",
					"bg-zinc-100",
					"text-zinc-950",
					"border-zinc-200",
					"hover:bg-zinc-200",
					"dark:border-zinc-800",
					"dark:bg-zinc-900",
					"hover:dark:bg-zinc-800",
				],
			},
		},
	}
)
