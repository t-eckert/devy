import User from "lib/user"

export interface PostMetadata {
	title: string
	author: User
	path: string
	tags: string[]
	updated: string
	likes: number
}

interface Post extends PostMetadata {
	markdown: string
	html: string
}

export default Post
