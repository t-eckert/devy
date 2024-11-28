import { redirect } from "@sveltejs/kit"
import { parseSessionToken } from "$lib/auth"
import type { Actions, PageServerLoad } from "./$types"
import { env } from "$env/dynamic/private"

interface Submission {
	userId: string
	username: string
	name: string
	repoUrl: string
	slug: string
	description?: string
}

export const actions = {
	default: async ({ request, fetch, locals }) => {
		const { token } = locals

		if (!token) {
			throw new Error("No token found")
		}

		const session = parseSessionToken(token).body

		const formData = await request.formData()
		const name = formData.get("Name")
		const repoUrl = formData.get("Repo")
		const slug = formData.get("Slug")
		if (!name || !repoUrl || !slug) {
			throw new Error("Missing required fields")
		}

		const submission: Submission = {
			userId: session.userId,
			username: session.username,
			name: name as string,
			repoUrl: repoUrl as string,
			slug: slug as string
		}
		console.log(submission)

		const resp = await fetch(`${env.API}/forms/new-blog`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(submission)
		})
		console.log(resp.status, resp.statusText)

		if (resp.ok) {
			redirect(307, "/" + slug)
		}
	}
} satisfies Actions

export const load: PageServerLoad = async ({ fetch, locals }) => {
	const { token } = locals

	if (!token) {
		return redirect(307, "/auth/login")
	}

	const session = parseSessionToken(token).body
	const resp = await fetch(`/api/users/${session.username}/github/repos`, {
		headers: {
			Authorization: `Bearer ${token}`
		}
	})

	return {
		repos: await resp.json()
	}
}
