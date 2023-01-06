import Post, { PostMetadata } from "./Post"
import mocks from "components/Article/Article.mocks"
import postMocks from "sections/Feed/Feed.mocks"
import Feed from "lib/feed"

export const getPost = (authorname: string, title: string): Post => {
	return {
		title,
		slug: "",
		author: {
			id: "7670827d-7b4f-4100-a4e3-d217079d5968",
			name: authorname,
			username: "t-eckert",
		},
		tags: [],
		updated: "",
		likes: 20,
		markdown: "",
		html: mocks.base.html,
	}
}

export const getPostsMetadataByFeed = (
	feed: Feed
): PostMetadata[] | undefined => {
	return postMocks.popularFeed.postsMetadata
}
