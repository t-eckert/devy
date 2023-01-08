import Post, { PostMetadata } from "./Post"
import { defaults } from "lib/feed"
import fixtures from "fixtures/post"


export const getPost = (authorname: string, slug: string): Post | null => {
  const post = fixtures.posts.filter((post) => post.author.username === authorname && post.slug === slug).at(0)
  if (post === undefined) {
    return null
  }
  return post
}

export const getPostsMetadataByFeed = (
  feedId: string
): PostMetadata[] => {
  if (!defaults.map(d => d.id).includes(feedId)) {
    throw "Not found"
  }

  const meta = fixtures.postsMetadata

  const popular = defaults.at(0)
  if (feedId === popular?.id) {
    meta.sort((a, b) => a.likes < b.likes ? -1 : 1)
    return meta
  }

  const newFeed = defaults.at(1)
  if (feedId === newFeed?.id) {
    meta.sort((a, b) => a.updatedAt < b.updatedAt ? -1 : 1)
    return meta
  }

  return []
}
