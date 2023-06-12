import db from "@/db"

export default interface Profile {
	username: string
	displayName: string
	avatarUrl?: string
}

export const getProfile = async (
	username: string
): Promise<Profile | undefined> => {
	const user = await db.user.findUnique({
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
