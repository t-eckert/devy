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
	"py-1",
	"px-2",
	"rounded-md",
	"border",
	"shadow-md",
	"border-neutral-darker",
	"dark:border-neutral-medium",
	"bg-neutral-lightest",
	"dark:bg-neutral-darker",
]
