import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Blog, Entry } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	return {
		props: {
			blog: await api.get<Blog>(`/blogs/${params.blog}`),
			entries: await api.get<Entry[]>(`/blogs/${params.blog}/entries`)
		}
	}
}
