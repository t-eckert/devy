export default interface Session {
	userId: string
	profileId: string
	username: string
	displayName: string
	avatarUrl?: string
	role: string
	status: string

	createdAt: Date
	exp: Date
}
