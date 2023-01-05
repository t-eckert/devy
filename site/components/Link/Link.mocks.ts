import { Props } from "./Link"

const base: Props = {
	href: "#",
	text: "Unstyled link",
}

const primary: Props = {
	...base,
	style: "primary",
	text: "Primary link",
}

const secondary: Props = {
	...base,
	style: "secondary",
	text: "Secondary link",
}

const button: Props = {
	...base,
	style: "button",
	text: "Button link",
}

const mocks = {
	base,
	primary,
	secondary,
	button,
}

export default mocks
