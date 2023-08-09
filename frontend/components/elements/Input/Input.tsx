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
		"rounded-md",
		"dark:bg-zinc-800",
		"focus:outline-none",
		"border",
		"dark:border-zinc-600",
	],
	{
		variants: {},
	}
)
