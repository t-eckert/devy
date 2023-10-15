import Profile from "@/models/Profile"
import User from "@/models/User"

export default interface Session {
	user: User
	profile: Profile
	token: any
}
