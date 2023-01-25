import Profile from "lib/profile"

// PostMetadata represents information about a given Post relevant to displaying a preview link in an index.
export interface PostMetadata {
  createdAt: string
  updatedAt?: string
  title: string
  slug: string
  author: Profile
  tags: string[]
  likes: number
}

// Post represents a blog post created by a user.
export interface Post extends PostMetadata {
  markdown: string
}
