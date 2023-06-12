import User from "./User"

export default interface Follow {
	follower: User
	followee: User
}

export const getFollowersByUser = async (username: string): Promise<User[]> => {
	return []
}

export const getFolloweesByUser = async (username: string): Promise<User[]> => {
	return []
}
