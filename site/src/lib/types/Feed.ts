import type Post from "./Post"
import type Blog from "./Blog"
import type Profile from "./Profile"

type Feed = { post: Post; blog: Blog; profile: Profile }[]

export default Feed
