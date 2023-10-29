type SeparatorItem = {
	type: "separator"
}
type LinkItem = {
	type: "link"
	label: string
	href: string
}
type ButtonItem = {
	type: "button"
	label: string
	onClick: () => void
}
type Item = SeparatorItem | LinkItem | ButtonItem

interface Props {
	items: Item[]
}

export default function Menu({}: Props) {
	return <section>Menu</section>
}
