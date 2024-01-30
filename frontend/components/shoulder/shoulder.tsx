// TODO maybe this should be a layout component

interface Props {
	children?: React.ReactNode
}

export default function Shoulder({ children }: Props) {
	return (
		<section className="flex-col w-64 gap-2 hidden sm:flex">
			{children}
		</section>
	)
}
