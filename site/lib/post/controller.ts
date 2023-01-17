import Post, { PostMetadata } from "./Post"
import { defaults } from "lib/feed"
import fixtures from "fixtures/post"
import { IS_LOCAL } from "config"
import client from "client"

export const getPost = async (
  authorname: string,
  slug: string
): Promise<Post | null> => {
  console.log(process.env)
  console.log(IS_LOCAL)
  if (IS_LOCAL) {
    return (
      fixtures.posts
        .filter(
          (post) => post.author.username === authorname && post.slug === slug
        )
        .at(0) || null
    )
  }

  const { data, error } = await client.from("post").select()
  if (error) {
    throw error
  }

  return (
    data
      .filter(
        (post) => post.author.username === authorname && post.slug === slug
      )
      .at(0) || null
  )
}

export const getPostsMetadataByFeed = (feedId: string): PostMetadata[] => {
  if (!defaults.map((d) => d.id).includes(feedId)) {
    throw "Not found"
  }

  const meta = fixtures.postsMetadata

  const popular = defaults.at(0)
  if (feedId === popular?.id) {
    meta.sort((a, b) => (a.likes < b.likes ? -1 : 1))
    return meta
  }

  const newFeed = defaults.at(1)
  if (feedId === newFeed?.id) {
    meta.sort((a, b) => (a.updatedAt < b.updatedAt ? -1 : 1))
    return meta
  }

  return []
}
