import { Post, createPost } from "@/lib/posts"

const posts: Post[] = [
	{
		title: "Hello World",
		markdown: "This is my first post!",
	},
	{
		title: "Hello Again",
		markdown: "This is my second post!",
	},
	{
		title: "Hello World",
		markdown: "This is my third post!",
	},
]

export const seedPosts = async () => {
	posts.forEach(async (post) => {
		await createPost(post)
	})
}
