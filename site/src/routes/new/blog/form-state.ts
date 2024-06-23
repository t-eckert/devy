import { writable } from "svelte/store"
import type { Writable } from "svelte/store"
import type { Repo } from "$lib/types"
import slug from "$lib/slug"

export interface FormState {
	blogName: string
	blogSlug: string
	repos: Repo[]
	selectedRepository: string
}

const formState: Writable<FormState> = writable({
	blogName: "",
	blogSlug: "",
	repos: [],
	selectedRepository: ""
})

export const hydrate = async (token: string) => {
	const resp = await fetch("/api/users/t-eckert/github/repos")
	console.log(resp)

	if (resp.status !== 200) {
		throw new Error(`Failed to fetch repos for user: ${resp.statusText}`)
	}

	const repos = (await resp.json()) as Repo[]
	formState.update((state) => ({ ...state, repos }))
}

export const setBlogName = (blogName: string) => {
	formState.update((state) => ({ ...state, blogName, blogSlug: slug(blogName) }))
}

export const setBlogSlug = (blogSlug: string) => {
	formState.update((state) => ({ ...state, blogSlug }))
}

export const setSelectedRepository = (selectedRepository: string) => {
	formState.update((state) => ({ ...state, selectedRepository }))
}

export default formState
