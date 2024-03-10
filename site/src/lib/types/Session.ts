import type User from "./User"
import type Profile from "./Profile"

export default interface Session {
	id: string

	metadata: {
		user: User
		profile: Profile
	}

	createdAt: string
	exp: number
}
