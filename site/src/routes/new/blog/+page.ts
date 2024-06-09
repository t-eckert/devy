import type { PageLoad } from "./$types"
import type { Repo } from "$lib/types"
import api from "$lib/api"

export const load: PageLoad = async (a) => {
	console.log(a)

	const repos = await api.get<Repo[]>("/users/t-eckert/github/repos")

	return {
		repos
	}
}
