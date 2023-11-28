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

	"border-neutral+1",
	"dark:border-neutral-1",

	"bg-neutral+2",
	"dark:bg-neutral-2",
]
