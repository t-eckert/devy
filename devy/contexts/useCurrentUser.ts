import User from "@/models/User"

export default function useCurrentUser(): User {
	return {
		username: "test",
		displayName: "Test",
		githubId: "123456",
		githubUsername: "test",
	}
}
