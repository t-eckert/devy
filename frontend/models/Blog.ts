import Profile from "./Profile"
import Post from "./Post"

interface Blog {
	name: string
	slug: string
	profile: Profile
	posts: Post[]
}

export default Blog
