interface Blog {
	id: string

	profileId: string

	name: string
	slug: string
	description?: string

	createdAt?: string
	updateAt?: string
}

export default Blog
