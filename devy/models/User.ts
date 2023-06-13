import type Translator from "@/interfaces/Translator"

export default interface User {
	username: string
	displayName: string
	email: string
	avatarUrl?: string
	githubId?: string
	githubUsername?: string
}

export const userTranslator: Translator<any, User> = {
	toModel: (prisma) => ({
		username: prisma.username,
		displayName: prisma.name ?? prisma.username,
		email: prisma.email,
		avatarUrl: prisma.avatarUrl ?? undefined,
		githubId: "",
		githubUsername: "",
	}),
	toPrisma: (model) => ({
		username: model.username,
		email: model.email,
		name: model.displayName,
		avatarUrl: model.avatarUrl ?? undefined,
	}),
}
