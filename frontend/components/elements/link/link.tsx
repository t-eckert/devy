import { VariantProps, cva } from "cva"
import NextLink, { LinkProps } from "next/link"

import { styles as buttonStyles } from "@/components/elements/button"

interface Props extends LinkProps {
	children?: React.ReactNode
	className?: string
	variant?: VariantProps<typeof styles>
	asButton?: boolean
	buttonVariant?: VariantProps<typeof buttonStyles>
}

export default function Link({
	children,
	variant,
	className,
	asButton,
	buttonVariant,
	...props
}: Props) {
	// TODO this kind of sucks. I can use the next version of CVA to fix this
	if (asButton)
		return (
			<NextLink
				className={[buttonStyles(buttonVariant), className].join(" ")}
				{...props}
			>
				{children}
			</NextLink>
		)

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
