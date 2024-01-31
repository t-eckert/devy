interface Props {
	count: number
}

export default function Counter({ count }: Props) {
	return <>{format(count)}</>
}

function format(count: number): string {
	if (count < 1_000) {
		return count.toString()
	} else if (count < 10_000) {
		return (count / 1_000).toFixed(1) + "k"
	} else if (count < 1_000_000) {
		return (count / 1_000).toFixed(0) + "k"
	} else {
		return (count / 1_000_000).toFixed(1) + "M"
	}
}
