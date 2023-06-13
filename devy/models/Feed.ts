import Post from "./Post"

export default interface Feed {
	slug: string
	name: string
	posts: Post[]
}

export const getFeeds = () => {
	return feeds
}

export const getFeed = (slug: string) => {
	return feeds.find((feed) => feed.slug === slug)
}
