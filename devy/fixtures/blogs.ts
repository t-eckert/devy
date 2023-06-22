import Blog from "@/models/Blog"

import users from "./users"
import posts from "./posts"

const blogs: Blog[] = [
	{
		slug: "ml-blog",
		name: "Machine Learning Blog",
		author: users[0],
		posts: [posts[0]],
	},
	{
		slug: "web-dev-blog",
		name: "Web Development Blog",
		author: users[1],
		posts: [posts[1]],
	},
	{
		slug: "bata-dlog",
		name: "Data Blog",
		author: users[0],
		posts: [posts[2]],
	},
	{
		slug: "security-blog",
		name: "Security Blog",
		author: users[1],
		posts: [posts[3]],
	},
]

export default blogs
