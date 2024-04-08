import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Entry } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	return {
		props: {
			entry: await api.get<Entry>(`/blogs/${params.blog}/entries/${params.post}`)
		}
	}
}
