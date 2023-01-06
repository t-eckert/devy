import User from "lib/user"

export interface PostMetadata {
	title: string
	slug: string
	author: User
	tags: string[]
	updated: string
	likes: number
}

interface Post extends PostMetadata {
	markdown: string
	html: string
}

export default Post
