export default interface Profile {
	id: string
	userId: string

	displayName: string
	avatarUrl?: string
	bio?: string
	website?: string

	createdAt: string
	updatedAt: string
}
