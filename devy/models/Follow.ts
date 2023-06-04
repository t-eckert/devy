import User from "./User"

export default interface Follow {
	follower: User
	followee: User
}
