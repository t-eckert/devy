export interface Content {
	metadata: Feed
	status: "loaded" | "loading" | "error"
	posts: Post[]
	page: number
	pageSize: number
}
