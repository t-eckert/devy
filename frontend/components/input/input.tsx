import { VariantProps, cva } from "cva"
import { InputHTMLAttributes } from "react"

interface Props extends InputHTMLAttributes<HTMLInputElement> {
	variant?: VariantProps<typeof styles>
}

export default function Input({ variant, ...props }: Props) {
	return <input className={styles(variant)} {...props}></input>
}

const styles = cva(
	[
		"px-2",
		"py-1",
		"rounded-md",
		"border",
		"transition-all",
		"border-zinc-400",
		"bg-zinc-50",
		"focus:outline-none",
		"dark:bg-zinc-800",
		"dark:border-zinc-600",
		"dark:text-zinc-50",
		"dark:focus:ring-offset-zinc-400",
	],
	{
		variants: {},
	}
)
