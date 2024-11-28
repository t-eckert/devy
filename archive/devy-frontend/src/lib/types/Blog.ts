export default interface Blog {
	id: string

	profileId: string
	userId: string

	authorUsername: string
	authorDisplayName: string

	name: string
	slug: string
	description?: string

	repoUrl: string

	createdAt: string
	updateAt: string
}
