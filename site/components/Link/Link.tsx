import NextLink from "next/link"

export interface Props {
	href: string
	style?: "primary" | "secondary" | "button"
	text?: string
	className?: string
	children?: React.ReactNode
}

const Link: React.FC<Props> = ({ href, style, text, className, children }) => {
	const classes: string[] = ["transition-all"]

	switch (style) {
		case "primary":
			classes.push("font-medium hover:underline")
		case "secondary":
			classes.push("font-normal hover:underline")
		case "button":
			classes.push("font-medium")
		default:
			classes.push("")
	}

	if (className) {
		classes.push(className)
	}

	return (
		<NextLink href={href} className={classes.join(" ")}>
			{text}
			{children}
		</NextLink>
	)
}

export default Link
