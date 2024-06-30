export default interface Post {
	id: string
	blog_id: string

	slug: string
	title: string
	body: string
	likes: number

	createdAt: string
	updatedAt: string
}
