import { VariantProps, cva } from "cva"
import NextLink, { LinkProps } from "next/link"

interface Props extends LinkProps {
	children?: React.ReactNode
	variant?: VariantProps<typeof styles>
}

export default function Link({ children, variant, ...props }: Props) {
	return (
		<NextLink className={styles(variant)} {...props}>
			{children}
		</NextLink>
	)
}

const styles = cva(
	[
		"pointer-cursor",
		"rounded-sm",
		"text-zinc-950",
		"transition-all",
		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-500",
		"focus:ring-offset-4",
		"dark:text-zinc-50",
		"dark:focus:ring-zinc-200",
	],
	{
		variants: {
			underline: {
				true: [
					"underline",
					"decoration-zinc-700",
					"dark:decoration-zinc-200",
				],
				false: "no-underline",
			},
		},
	}
)
