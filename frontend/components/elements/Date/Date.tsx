interface Props {
	date: string
	compareTo?: string
}

export default function DateComponent({ date, compareTo }: Props) {
	const basis = compareTo || new Date().toISOString()
	const delta = new Date(basis).getTime() - new Date(date).getTime()

	return <time>{name(delta) || new Date(date).toLocaleDateString()}</time>
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
		case delta < 1000 * 60 * 60 * 24 * 7:
			return `${Math.floor(delta / (1000 * 60 * 60 * 24))} days ago`
		case delta < 1000 * 60 * 60 * 24 * 30:
			return `${Math.floor(delta / (1000 * 60 * 60 * 24 * 7))} weeks ago`
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
