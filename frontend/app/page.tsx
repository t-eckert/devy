import FeedComponent from "@/components/dynamic/Feed"
import Shoulder from "@/components/segments/Shoulder"
import Changelog from "@/components/segments/Changelog"

import api from "@/api"
import { fetchChangelog } from "@/changelog"

import Feed from "@/models/Feed"
import Post from "@/models/Post"

export default async function Home() {
  const newFeed = await api.get<Feed>("/feeds/new", 60)
  const newFeedPosts = await api.get<Post[]>(`/feeds/new/posts`, 60)
  const changelog = await fetchChangelog()

  if (!newFeed || !newFeedPosts) return <NotFound />

  // TODO change this in the feed component to take the feed object as the key in the map.
  const feeds = [
    {
      feedMeta: newFeed,
      posts: newFeedPosts,
    },
  ]

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <FeedComponent feeds={feeds} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </main>
  )
}

const NotFound = () => (
  <main className="mx-auto flex flex-row w-full max-w-6xl">
    <span>Unable to load feeds</span>
  </main>
)
