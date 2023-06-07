import { prisma } from "@/db"

export default interface Profile {
	username: string
	displayName: string
	avatarUrl?: string
}

export const getProfile = async (
	username: string
): Promise<Profile | undefined> => {
	const user = await prisma.user.findUnique({
		where: {
			username,
		},
	})

	if (!user) {
		return undefined
	}

	return {
		username: user.username,
		displayName: user.name ?? user.username,
		avatarUrl: user.avatarUrl ?? undefined,
	}
}
