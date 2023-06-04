import Post from "./Post"

export default interface Feed {
	id: string
	name: string
	posts: Post[]
}
