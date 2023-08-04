export default interface Post {
	id: string
	slug: string
	title: string
	markdown: string
	tags: string[]
	author: {
		name: string
		id: string
		email: string
	}
	likes: number
	likedByUser: boolean
}
