interface Props {
	children?: React.ReactNode
	className?: string
}

export default function Card({ children, className }: Props) {
	return (
		<section className={[className, ...card].join(" ")}>{children}</section>
	)
}

const card = [
	"rounded-md",
	"border",
	"shadow-md",

	"border-neutral+1",
	"dark:border-neutral-1",

	"bg-neutral+3",
	"dark:bg-neutral-3",
]
