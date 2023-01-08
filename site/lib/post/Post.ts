import User from "lib/user"

// PostMetadata represents information about a given Post relevant to displaying a preview link in an index.
export interface PostMetadata {
  id: string
  createdAt: string
  updatedAt: string
  title: string
  slug: string
  author: User
  tags: string[]
  likes: number
}

// Post represents a blog post created by a user.
interface Post extends PostMetadata {
  markdown: string
}

export default Post
