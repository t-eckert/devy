import Feed from "./Feed"

export const getFeed = (id: string): Feed => {
	return {
		id,
		name: "Popular",
	}
}
