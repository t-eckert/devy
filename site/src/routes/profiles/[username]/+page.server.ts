import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { User, Blog, Profile, Entry } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	const user = await api.get<User>(`/users/${params.username}`)
	const profile = await api.get<Profile>(`/profiles/${params.username}`)
	const blogs = await api.get<Blog[]>(`/profiles/${params.username}/blogs`)
	const entries = await api.get<Entry[]>(`/profiles/${params.username}/entries`)

	return {
		props: {
			profile,
			blogs,
			user,
			entries
		}
	}
}
