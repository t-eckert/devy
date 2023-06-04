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

const feeds = [
	{
		slug: "popular",
		name: "Popular",
		posts: [
			{
				title: "Post 1",
				markdown: "Post 1 content",
			},
			{
				title: "Post 2",
				markdown: "Post 2 content",
			},
			{
				title: "Post 3",
				markdown: "Post 3 content",
			},
			{
				title: "Post 4",
				markdown: "Post 4 content",
			},
		],
	},
	{
		slug: "new",
		name: "New",
		posts: [
			{
				title: "Post 3",
				markdown: "Post 3 content",
			},
			{
				title: "Post 4",
				markdown: "Post 4 content",
			},
			{
				title: "Post 1",
				markdown: "Post 1 content",
			},
			{
				title: "Post 2",
				markdown: "Post 2 content",
			},
		],
	},
]
