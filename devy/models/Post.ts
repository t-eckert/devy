import User from "./User"

export default interface Post {
	slug: string
	blog: string
	author: User
	title: string
	published: string
	updated?: string
	tags: string[]
	likes: number
	markdown: string
}
