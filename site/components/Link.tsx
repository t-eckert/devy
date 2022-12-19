import NextLink from "next/link"

type Props = {
	href: string
	className?: string
	children?: React.ReactNode
}

const Link: React.FC<Props> = ({ href, className, children }) => {
	return (
		<NextLink
			href={href}
			className={["font-medium transition-all hover:underline", className].join(
				" "
			)}
		>
			{children}
		</NextLink>
	)
}

export default Link
