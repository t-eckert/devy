import { Props } from "./Nav"

const loggedOut: Props = {
	user: undefined,
}

const loggedIn: Props = {
	user: {
		id: "d04372df-ae01-40fc-bfec-0568243c2f3e",
		name: "Thomas Eckert",
		username: "thomaseckert",
	},
}

const mocks = {
	loggedOut,
	loggedIn,
}

export default mocks
