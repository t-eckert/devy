import { setContext, getContext } from "svelte"

type ReposLoadState = "not-loaded" | "loading" | "loaded" | "error"

export class FormState {
	repos = []
	reposLoadState = "not-loaded"
	selectedRepo = ""

	constructor() {}
}
