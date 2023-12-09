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

const styles = cva(["pointer-cursor", "transition-all"], {
	variants: {
		underline: {
			true: "underline decoration-neutral underline-offset-2",
			false: "no-underline",
		},
		styled: {
			true: "rounded-sm",
			false: []
		}
	},
	defaultVariants: {
		underline: true,
		styled: true,
	}
})
