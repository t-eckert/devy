import { useEffect } from "react"

import useSession from "@/lib/auth/useSession"
import useStore from "@/lib/useStore"
import { useQuery } from "@tanstack/react-query"

import { Session } from "@/lib/auth"
import { Dispatch, SetStateAction } from "react"

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
	selectedRepo: GitHubRepo | null
	setSelectedRepo: (repo: GitHubRepo) => void

	blogName: string
	setBlogName: Dispatch<SetStateAction<string>>

	isSubmittable: boolean
	onSubmit: () => void

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

	/*
	const [limit, setLimit] = useState(9)
	const [selected, setSelected] = useState<GitHubRepo | null>(null)
	const [blogName, setBlogName] = useState<string>("")
	const [isSubmittable, setIsSubmittable] = useState<boolean>(false)

	useEffect(() => {
		setIsSubmittable(blogName.length > 0 && selected !== null)
	}, [blogName, selected])

	const onSubmit = async (e: any) => {
		e.preventDefault()
		if (!selected) return
		console.log(e)

		await api.post("/v1/blogs", {
			username: session?.session.user.username,
			name: selected.name,
			repoUrl: selected.html_url,
			githubId: selected.id,
			githubName: selected.name,
			metadata: selected,
		})

		// if (res.status === 200) {
		//   const blog = (await res.json()) as Blog
		//   window.location.href = `/blogs/${blog.id}`
		// }
	}
	*/

	return {
		repos: repos ?? [],
		isLoadingRepos,
		selectedRepo: null,
		setSelectedRepo: () => {},
		blogName: "",
		setBlogName: () => {},
		isSubmittable: false,
		onSubmit: () => {},
		session,
		status,
	}
}
