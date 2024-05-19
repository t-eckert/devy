import type User from "./User"
import type Profile from "./Profile"

export default interface Session {
	user: User
	profile: Profile
}
