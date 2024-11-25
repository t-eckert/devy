export default interface Profile {
	id: string
	userId: string

	displayName: string
	avatarUrl?: string
	bio?: string
	websiteUrl?: string
	twitterUsername?: string
	githubUsername?: string

	status: "active"
	visibility: "public" | "private"

	isDeleted: boolean
	isLocked: boolean
	isFeatured: boolean
	isBot: boolean
	isSponsor: boolean

	createdAt: string
	updatedAt: string
}
