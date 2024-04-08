import type { PageServerLoad } from "./$types"
import api from "$lib/api"
import type { Profile } from "$lib/types"

export const load: PageServerLoad = async ({ params }) => {
	const profile = await api.get<Profile>(`/profiles/${params.username}`)

	return {
		props: {
			profile
		}
	}
}
