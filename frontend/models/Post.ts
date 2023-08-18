import Profile from "./Profile"

export default interface Post {
	id: number
	slug: string
	title: string
	content: string
	tags: string[]
	author: Profile
	createdAt: string
	updatedAt: string
	likes: number
}
