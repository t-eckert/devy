export default interface Session {
	userId: string
	username: string
	role: string
	status: string

	displayName?: string
	avatarUrl?: string
}
