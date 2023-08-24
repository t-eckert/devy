export default interface Post {
	id: number
	slug: string

	blogSlug: string
	blogName: string
	authorSlug: string
	authorName: string

	title: string
	content: string
	tags: string[]
	createdAt: string
	updatedAt: string
	likes: number
}
