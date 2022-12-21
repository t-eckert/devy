import NextLink from "next/link"

export interface Props {
	href: string
	style?: "primary" | "secondary" | "button"
	text?: string
	className?: string
	children?: React.ReactNode
}

const Link: React.FC<Props> = ({ href, style, text, className, children }) => {
	const classes: string[] = ["transition-all hover:underline"]

	switch (style) {
		case "primary":
			classes.push("font-medium")
		case "secondary":
			classes.push("font-normal")
		default:
			classes.push("font-medium")
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
