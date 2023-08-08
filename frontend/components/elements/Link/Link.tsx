import { VariantProps, cva } from "cva"
import NextLink from "next/link"

interface Props {
	href: string
	children: React.ReactNode
	style?: VariantProps<typeof styles>
}

export default function Link({ href, children, style }: Props) {
	return (
		<NextLink href={href} className={styles(style)}>
			{children}
		</NextLink>
	)
}

const styles = cva(
	[
		"pointer-cursor",
		"rounded-md",
		"focus:outline-none",
		"focus:ring-1",
		"focus:ring-zinc-200",
	],
	{
		variants: {
			underline: {
				true: "underline",
				false: "no-underline",
			},
		},
	}
)
