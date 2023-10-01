import Shoulder from "@/components/segments/Shoulder"
import Changelog from "@/components/segments/Changelog"
import HomeFeed, { FeedContent } from "@/lib/feed/HomeFeed"

import api from "@/lib/api"
import { fetchChangelog } from "@/lib/changelog"

import Feed from "@/models/Feed"
import Post from "@/models/Post"

export default async function Home() {
  const feeds = await fetchFeeds(0, 25)
  const changelog = await fetchChangelog()

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HomeFeed feeds={feeds} defaultSelected={"new"} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </main>
  )
}

const fetchFeeds = async (
  page: number,
  pageSize: number
): Promise<FeedContent[]> => {
  const feeds: FeedContent[] = []

  const newFeed = await api.get<Feed>("/feeds/new", 600)
  const newFeedPosts = await api.get<Post[]>(
    `/feeds/new/posts?offset=${page / pageSize}&limit=${pageSize}`,
    600
  )
  if (newFeed)
    feeds.push({
      metadata: newFeed,
      status: "loaded",
      posts: newFeedPosts || [],
      page,
      pageSize,
    })

  const popularFeed = await api.get<Feed>("/feeds/popular", 600)
  const popularFeedPosts = await api.get<Post[]>(
    `/feeds/popular/posts?offset=${page / pageSize}&limit=${pageSize}`,
    600
  )
  if (popularFeed)
    feeds.push({
      metadata: popularFeed,
      status: "loaded",
      posts: popularFeedPosts || [],
      page,
      pageSize,
    })

  return feeds
}
