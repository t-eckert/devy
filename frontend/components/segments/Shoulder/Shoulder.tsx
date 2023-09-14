interface Props {
	children?: React.ReactNode
}

export default function Shoulder({ children }: Props) {
	return (
		<section className="flex flex-col w-64 gap-2 hidden sm:visible">
			{children}
		</section>
	)
}
