import { Props } from "./Link"

const base: Props = {
	href: "#",
	text: "Link",
}

const primary: Props = {
	...base,
	style: "primary",
}

const secondary: Props = {
	...base,
	style: "secondary",
}


const mocks = {
	base,
	primary,
	secondary,
}

export default mocks
