import type { PageServerLoad, Actions } from "./$types"
import api from "$lib/api"

export type Status = "logged-in" | "logged-out"

type NewBlogRequest = {
	name: string
	repoName: string
}

export const actions: Actions = {
	default: async ({ request }) => {
		const formData = await request.formData()
		const name = formData.get("name")
		const repoName = formData.get("githubRepo")

		if (typeof name !== "string" || typeof repoName !== "string") {
			return {
				status: 400,
				body: "Invalid request"
			}
		}

		const newBlogRequest: NewBlogRequest = {
			name,
			repoName
		}

		const resp = await api.post("/forms/new-blog", newBlogRequest)

		if (resp.status === 200) {
			return {
				status: 200,
				body: "Blog created"
			}
		}

		return {
			status: 500,
			body: "Internal server error"
		}
	}
}

export const load: PageServerLoad = async ({ locals }) => {
	if (locals.session === undefined) {
		return {
			props: {
				githubRepos: [],
				status: "logged-out"
			}
		}
	}

	const username = locals.session.metadata.user.username

	const resp = await fetch(
		`https://api.github.com/users/${username}/repos?per_page=100&sort=updated`
	)

	return {
		props: {
			githubRepos: await resp.json(),
			status: "logged-in"
		}
	}
}
