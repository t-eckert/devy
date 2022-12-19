import User from "./User"

interface PostMetadata {
	title: string
	author: User
	path: string
	tags: string[]
	likes: number
}

export default PostMetadata
