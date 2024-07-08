import type { PageLoad } from "./$types"
import type { User, Blog, Profile, Entry } from "$lib/types"
import { error } from "@sveltejs/kit"

export const load: PageLoad = async ({ fetch, params }) => {
	const userResp = await fetch(`/api/users/${params.username}`)
	if (!userResp.ok) {
		throw error(userResp.status, "User not found")
	}
	const user: User = await userResp.json()

	const profileResp = await fetch(`/api/profiles/${params.username}`)
	if (!profileResp.ok) {
		throw error(profileResp.status, "Profile not found")
	}
	const profile: Profile = await profileResp.json()

	const blogsResp = await fetch(`/api/profiles/${params.username}/blogs`)
	if (!blogsResp.ok) {
		throw error(blogsResp.status, "Blogs not found")
	}
	const blogs: Blog[] = await blogsResp.json()

	const entriesResp = await fetch(`/api/profiles/${params.username}/entries`)
	if (!entriesResp.ok) {
		throw error(entriesResp.status, "Entries not found")
	}
	const entries: Entry[] = await entriesResp.json()

	return {
		props: {
			profile,
			blogs,
			user,
			entries
		}
	}
}
