import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Blog, Post } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	return {
		props: {
			blog: await api.get<Blog>(`/blogs/${params.blog}`),
			posts: await api.get<Post[]>(`/blogs/${params.blog}/posts`)
		}
	}
}
