import { writable } from "svelte/store"
import type { Writable } from "svelte/store"
import type { Repo } from "$lib/types"
import slug from "$lib/slug"

export interface FormState {
	blogName: string
	blogSlug: string
	repos: Repo[]
	repoLoadState: "not-loaded" | "loading" | "loaded" | "error"
	selectedRepository: string
}

const formState: Writable<FormState> = writable({
	blogName: "",
	blogSlug: "",
	repos: [],
	repoLoadState: "not-loaded",
	selectedRepository: ""
})

export const hydrate = async (username: string) => {
	formState.update((state) => ({ ...state, repoLoadState: "loading" }))
	const resp = await fetch(`/api/users/${username}/github/repos`)

	if (resp.status !== 200) {
		formState.update((state) => ({ ...state, repoLoadState: "error" }))
	}

	const repos = (await resp.json()) as Repo[]
	formState.update((state) => ({ ...state, repos, repoLoadState: "loaded" }))
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

export function isSubmitable() {
	return false
}

export default formState
