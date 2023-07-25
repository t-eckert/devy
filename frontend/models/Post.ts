export default interface Post {
	slug: string
	title: string
	markdown: string
	tags: string[]
	author: string
	likes: number
	likedByUser: boolean
}
