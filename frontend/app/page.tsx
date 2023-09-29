import Shoulder from "@/components/segments/Shoulder"
import Changelog from "@/components/segments/Changelog"
import HomeFeed, { FeedContent } from "@/lib/feed/HomeFeed"

import api from "@/lib/api"
import { fetchChangelog } from "@/lib/changelog"

import Feed from "@/models/Feed"
import Post from "@/models/Post"

export default async function Home() {
  const feeds = await fetchFeeds(0, 10)
  const changelog = await fetchChangelog()

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HomeFeed feeds={feeds} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </main>
  )
}

const fetchFeeds = async (
  offset: number,
  pageSize: number
): Promise<Map<Feed, FeedContent>> => {
  const newFeed = await api.get<Feed>("/feeds/new", 600)
  const newFeedPosts = await api.get<Post[]>(
    `/feeds/new/posts?offset=${offset}&pageSize=${pageSize}`,
    600
  )

  if (newFeed === null) {
    return new Map([])
  }

  return new Map([
    [
      newFeed,
      {
        status: !(newFeed || newFeedPosts) ? "error" : "loaded",
        posts: newFeedPosts || [],
        offset,
        pageSize,
      },
    ],
  ])
}
