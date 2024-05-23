import { writable } from "svelte/store"
import type { Like } from "$lib/types"

export interface LikesStore {
	// The id of entries liked by the user.
	likes: Set<string>
}

const likesStore = writable<LikesStore>({
	likes: new Set()
})

export const hydrate = async (username: string) => {
	const resp = await fetch(`/api/likes/${username}`)
	const userLikes: Like[] = await resp.json()
	const likes = new Set(userLikes.map((like) => like.postId))
	likesStore.set({ likes })
}

export default likesStore
