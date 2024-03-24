import type { PageServerLoad, Actions } from "./$types"
import api from "$lib/api"

export type Status = "logged-in" | "logged-out"

type NewBlog = {
	username: string
	name: string
	repoUrl: string
}

export const actions: Actions = {
	default: async ({ request, locals }) => {
		if (locals.session === undefined) {
			return {
				status: 401,
				body: "Unauthorized"
			}
		}

		const formData = await request.formData()
		const name = formData.get("blog-name")
		const repoUrl = formData.get("repo-url")
		if (typeof name !== "string" || typeof repoUrl !== "string") {
			return {
				status: 400,
				body: "Invalid request"
			}
		}

		const newBlogRequest: NewBlog = {
			username: locals.session.metadata.user.username,
			name,
			repoUrl
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
