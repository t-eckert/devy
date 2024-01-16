import { useQuery } from "@tanstack/react-query"

import GitHubRepo from "./GitHubRepo"

export default function useUserRepos(username?: string) {
	return useQuery({
		queryKey: ["repos"],
		queryFn: async () => {
			if (!username) return []

			const res = await fetch(
				`https://api.github.com/users/${username}/repos?per_page=100&sort=updated`,
				{
					next: {
						revalidate: 60 * 3,
					},
				}
			)
			const raw = (await res.json()) as GitHubRepo[]
			return Array.from(raw).filter((repo: GitHubRepo) => !repo.fork)
		},
	})
}
