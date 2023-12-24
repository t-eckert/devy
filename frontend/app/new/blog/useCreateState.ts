import { useEffect } from "react"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import { Session } from "@/lib/auth"

interface GitHubRepo {
	id: number
	name: string
	html_url: string
	description: string
	fork: boolean
	language?: string
}

interface CreateState {
	repos: GitHubRepo[]
	isLoadingRepos: boolean

	session?: Session
	status?: string
}

export default function useCreateState(): CreateState {
	const sessionStore = useStore(useSession, (state) => state)
	const session = sessionStore?.session
	const status = sessionStore?.status

	let {
		data: repos,
		isLoading: isLoadingRepos,
		refetch,
	} = useQuery({
		queryKey: ["repos"],
		queryFn: async () => {
			if (status !== "logged-in") return []

			const res = await fetch(
				`https://api.github.com/users/${session?.user?.githubUsername}/repos?per_page=100&sort=updated`
			)
			const raw = (await res.json()) as GitHubRepo[]
			return Array.from(raw).filter((repo: GitHubRepo) => !repo.fork)
		},
	})

	useEffect(() => {
		refetch()
	}, [session])

	return {
		repos: repos ?? [],
		isLoadingRepos,

		session,
		status,
	}
}
