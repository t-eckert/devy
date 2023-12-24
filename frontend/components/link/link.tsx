import { VariantProps, cva } from "cva"
import NextLink, { LinkProps } from "next/link"

interface Props extends LinkProps {
	children?: React.ReactNode
	className?: string
	variant?: VariantProps<typeof styles>
}

export default function Link({
	children,
	variant,
	className,
	...props
}: Props) {
	return (
		<NextLink className={[styles(variant), className].join(" ")} {...props}>
			{children}
		</NextLink>
	)
}

const styles = cva(
	[
		"pointer-cursor",
		"transition-all",

		"focus-visible:outline-none",
		"focus-visible:ring-1",
		"focus-visible:ring-neutral-1",
		"focus-visible:dark:ring-neutral+1",
	],
	{
		variants: {
			underline: {
				true: "underline decoration-neutral underline-offset-2 ",
				false: "no-underline",
			},
			styled: {
				true: [
					"rounded-sm visited:decoration-purple-primary",
					"visited:text-purple-primary",
					"visited:dark:text-purple-primary",
				],
				false: [],
			},
		},
		defaultVariants: {
			underline: true,
			styled: true,
		},
	}
)
