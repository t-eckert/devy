export default interface Post {
	id: string
	slug: string

	blogName: string
	blogSlug: string
	authorName: string
	authorUsername: string

	title: string
	content: string
	tags: string[]
	createdAt: string
	updatedAt: string
	likes: number
}
