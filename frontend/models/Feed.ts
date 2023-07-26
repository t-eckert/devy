import Filter from "./Filter"
import Post from "./Post"

export default interface Feed {
	id: string
	slug: string
	name: string
	filters: Filter[]
	posts: Post[]
}
