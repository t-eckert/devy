import { VariantProps, cva } from "cva"

interface Props {
	children: React.ReactNode
	style?: VariantProps<typeof styles>
}

export default function Button({ children, style }: Props) {
	return <button className={styles(style)}>{children}</button>
}

const styles = cva(
	[
		"px-2",
		"pointer-cursor",
		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-200",
		"rounded-md",
	],
	{
		variants: {},
	}
)
