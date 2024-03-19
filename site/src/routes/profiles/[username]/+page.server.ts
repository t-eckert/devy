import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Profile, Post, Blog } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	return {
		props: {
			profile: await api.get<Profile>(`/profiles/${params.username}`),
			posts: await api.get<Post[]>(`/profiles/${params.username}/posts`),
			blogs: await api.get<Blog[]>(`/profiles/${params.username}/blogs`)
		}
	}
}
