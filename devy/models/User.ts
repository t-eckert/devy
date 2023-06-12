import Profile from "./Profile"

export default interface User extends Profile {
	githubId?: string
	githubUsername?: string
}

