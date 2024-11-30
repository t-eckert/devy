import db from "./db"
import { post } from "./db/schema"

export async function getFeed(type: string, page: number) {
  switch (type) {
    case "recent":
      return await recentFeed(page)
    case "popular":
      return await popularFeed(page)
    default:
      throw new Error(`Invalid feed type: ${type}`)
  }
}

async function recentFeed(page: number) {
  return {
    name: "Recent",
    page,
    posts: await db.select().from(post).orderBy(post.createdAt).limit(10).offset(page * 10)
  }
}

async function popularFeed(page: number) {
  return {
    name: "Popular",
    page,
    posts: await db.select().from(post).orderBy(post.likeCount).limit(10).offset(page * 10)
  }
}
