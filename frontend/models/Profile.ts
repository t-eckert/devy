export default interface Profile {
	id: string
	user_id: string

	displayName: string
	avatarUrl?: string
	bio?: string
	website?: string

	createdAt: string
	updatedAt: string
}
