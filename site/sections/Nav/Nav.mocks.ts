import { Props } from "./Nav"

const loggedOut: Props = {
	path: "",
	user: undefined,
}

const loggedOutOnProfilePage: Props = {
	path: "smarshall",
	user: undefined,
}

const loggedOutOnArticlePage: Props = {
	path: "smarshall/this-is-a-test-article",
	user: undefined,
}

const loggedOutOnDeepFolderArticlePage: Props = {
	path: "smarshall/projects/my-project/this-is-a-test-article",
	user: undefined,
}

const loggedIn: Props = {
	...loggedOut,
	user: {
		id: "d04372df-ae01-40fc-bfec-0568243c2f3e",
		name: "Thomas Eckert",
		username: "thomaseckert",
	},
}

const mocks = {
	loggedOut,
	loggedOutOnProfilePage,
	loggedOutOnArticlePage,
	loggedOutOnDeepFolderArticlePage,
	loggedIn,
}

export default mocks
