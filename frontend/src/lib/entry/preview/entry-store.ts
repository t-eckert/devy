import { writable } from "svelte/store"
import type { Entry } from "$lib/types"

export interface EntryStore {
	entry?: Entry
}

const entryStore = writable<EntryStore>({
	entry: undefined
})

export const like = () => {
	entryStore.update((store) => {
		if (store.entry) {
			store.entry.likes += 1
		}
		return store
	})
}

export default entryStore
