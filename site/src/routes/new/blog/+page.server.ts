import type { PageServerLoad, Actions } from "./$types"
import api from "$lib/api"
import { redirect } from "@sveltejs/kit"
import type { Blog, Repo } from "$lib/types"

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

		const newBlogResponse = await api.post("/forms/new-blog", newBlogRequest)
		if (newBlogResponse.status !== 200) {
			return {
				status: newBlogResponse.status,
				body: "Unable to create blog"
			}
		}

		const newUploadResponse = await api.post("/uploads", { repo: repoUrl })
		if (newUploadResponse.status !== 200) {
			return {
				status: newUploadResponse.status,
				body: "Unable to trigger upload"
			}
		}

		// Redirect to the new blog
		const resp = (await newBlogResponse.json()) as { blog: Blog; repo: Repo }
		throw redirect(303, `/${resp.blog.slug}`)
	}
}

export const load: PageServerLoad = async ({ locals }) => {
	console.log("locals", locals)

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
