import User from "./User"

interface PostMetadata {
	title: string
	author: User
	url: URL
	tags: string[]
}

export default PostMetadata
