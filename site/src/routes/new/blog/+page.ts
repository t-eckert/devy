import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ fetch }) => {
	const res = await fetch("/api/users/t-eckert/github/repos")
	if (!res.ok) {
		throw new Error("Failed to fetch repos")
	}

	const repos = await res.json()

	return {
		repos
	}
}
