interface Props {
	date: string
	compareTo?: string
	className?: string
}

export default function RelativeDate({ date, compareTo, className }: Props) {
	const basis = compareTo || new Date().toISOString()
	const delta = new Date(basis).getTime() - new Date(date).getTime()

	return <time className={className}>{name(delta) || new Date(date).toLocaleDateString()}</time>
}

const name = (delta: number): Option<string> => {
	switch (true) {
		case delta < 0:
			return "In the future"
		case delta < 1000:
			return "Now"
		case delta < 1000 * 60:
			return `${Math.floor(delta / 1000)} seconds ago`
		case delta < 1000 * 60 * 60:
			return `${Math.floor(delta / (1000 * 60))} minutes ago`
		case delta < 1000 * 60 * 60 * 24:
			return `${Math.floor(delta / (1000 * 60 * 60))} hours ago`
		case delta < 1000 * 60 * 60 * 24 * 2:
			return "Yesterday"
		case delta < 1000 * 60 * 60 * 24 * 7:
			return `${Math.floor(delta / (1000 * 60 * 60 * 24))} days ago`
		case delta < 1000 * 60 * 60 * 24 * 30:
			const weeks = Math.floor(delta / (1000 * 60 * 60 * 24 * 7))
			return `${weeks} week${weeks === 1 ? "" : "s"} ago`
		case delta < 1000 * 60 * 60 * 24 * 365:
			return `${Math.floor(
				delta / (1000 * 60 * 60 * 24 * 30)
			)} months ago`
		case delta < 1000 * 60 * 60 * 24 * 365 * 100:
			return `${Math.floor(
				delta / (1000 * 60 * 60 * 24 * 365)
			)} years ago`
		default:
			return null
	}
}
