import FeedConfig from "./FeedConfig"
import Post from "./Post"

export default interface Feed {
	feedConfig: FeedConfig
	posts: Post[]
}
