import { Post, PostMetadata } from "./Post"
import {
	getPostByPath,
	getPostsMetadataByFeed,
	getPostsMetadataByPopularity,
	getNewPostsMetadata,
} from "./controller"

export type { PostMetadata }
export {
	getPostByPath,
	getPostsMetadataByFeed,
	getPostsMetadataByPopularity,
	getNewPostsMetadata,
}
export default Post
