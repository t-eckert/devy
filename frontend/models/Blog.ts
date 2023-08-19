import Profile from "./Profile"
import Post from "./Post"

interface Blog {
	name: string
	slug: string
	profile: Profile
	posts: Post[]
	likedPosts: Post[]
	bookmarkdedPosts: Post[]
}

export default Blog
