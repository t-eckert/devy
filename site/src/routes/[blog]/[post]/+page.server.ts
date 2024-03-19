import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Post } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	return {
		props: {
			post: await api.get<Post>(`/v1/blogs/${params.blog}/posts/${params.post}`)
		}
	}
}
