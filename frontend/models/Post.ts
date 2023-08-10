import Profile from "./Profile"

export default interface Post {
	id: string
	slug: string
	title: string
	markdown: string
	tags: string[]
	author: Profile
	likes: number
}
