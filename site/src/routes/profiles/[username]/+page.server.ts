import type { PageServerLoad } from "./$types"
import api from "$lib/api"
<<<<<<< HEAD
import type { User, Blog, Profile, Entry } from "$lib/types"
=======
import type { User, Blog, Profile } from "$lib/types"
>>>>>>> d458d24 (site,router: link to profile on calling card)

export const load: PageServerLoad = async ({ params }) => {
	const user = await api.get<User>(`/users/${params.username}`)
	const profile = await api.get<Profile>(`/profiles/${params.username}`)
	const blogs = await api.get<Blog[]>(`/profiles/${params.username}/blogs`)
<<<<<<< HEAD
	const entries = await api.get<Entry[]>(`/profiles/${params.username}/entries`)
=======
	const user = await api.get<User>(`/users/${params.username}`)
>>>>>>> d458d24 (site,router: link to profile on calling card)

	return {
		props: {
			profile,
			blogs,
<<<<<<< HEAD
			user,
			entries
=======
			user
>>>>>>> d458d24 (site,router: link to profile on calling card)
		}
	}
}
