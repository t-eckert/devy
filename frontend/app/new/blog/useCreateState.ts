import { useEffect } from "react"
import { useQuery } from "@tanstack/react-query"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { Session } from "@/lib/auth"
import { Blog } from "@/models"
import api from "@/lib/api"

import GitHubRepo from "./GitHubRepo"

interface CreateState {
	repos?: GitHubRepo[]
	isLoadingRepos: boolean

	userBlogs?: Blog[]
	isLoadingUserBlogs: boolean

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
		refetch: refetchRepos,
	} = useQuery({
		queryKey: ["repos"],
		queryFn: async () => {
			if (status !== "logged-in") return []

			const res = await fetch(
				`https://api.github.com/users/${session?.user?.githubUsername}/repos?per_page=100&sort=updated`,
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

	let {
		data: userBlogs,
		isLoading: isLoadingUserBlogs,
		refetch: refetchUserBlogs,
	} = useQuery({
		queryKey: ["userBlogs"],
		queryFn: async () => {
			if (status !== "logged-in") return []

			return await api.get<Blog[]>(
				`/v1/profiles/${session?.user?.username}/blogs`,
				60 * 3
			)
		},
	})

	useEffect(() => {
		refetchRepos()
		refetchUserBlogs()
	}, [session])

	return {
		repos,
		isLoadingRepos,

		userBlogs,
		isLoadingUserBlogs,

		session,
		status,
	}
}
