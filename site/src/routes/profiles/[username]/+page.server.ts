import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { User, Blog, Profile } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	const profile = await api.get<Profile>(`/profiles/${params.username}`)
	const blogs = await api.get<Blog[]>(`/profiles/${params.username}/blogs`)
	const user = await api.get<User>(`/users/${params.username}`)

	return {
		props: {
			profile,
			blogs,
			user
		}
	}
}
