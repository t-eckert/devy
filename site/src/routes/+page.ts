import type { PageLoad } from "./$types"

export const load: PageLoad = () => {
	const feeds = ["new", "popular"]

	return {
		props: {
			feeds
		}
	}
}
