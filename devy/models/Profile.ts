export default interface Profile {
	username: string
	displayName: string
	avatar_url?: string
}

export const getProfile = async (
	username: string
): Promise<Profile | undefined> => {
	return profiles.find((profile) => profile.username === username)
}

const profiles = [
	{
		username: "test",
		displayName: "Test",
	},
	{
		username: "test2",
		displayName: "Test 2",
	},
]
