import Filter from "./Filter"
import Post from "./Post"

export default interface Feed {
	id: number
	slug: string
	name: string
	filters: Filter[]
	posts: Post[]
}
