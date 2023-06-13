import User from "@/models/User"

export default interface Follow {
	follower: User
	followee: User
}
