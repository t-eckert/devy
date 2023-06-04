export default interface Post {
	title?: string
	markdown?: string
}

export const getFeeds = () => {
	return posts
}

const posts = [
	{
		id: "1",
		name: "Feed 1",
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
		id: "2",
		name: "Feed 2",
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
