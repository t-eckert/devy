import feed from "@/controllers/feed"
import post from "@/controllers/post"
import Feed from "@/components/dynamic/Feed"

export default async function Home() {
  const newFeed = await feed.get.new()
  const newFeedPosts = (await post.get.byFeed(newFeed.id)) || []
  const popularFeed = await feed.get.popular()
  const popularFeedPosts = (await post.get.byFeed(newFeed.id)) || []

  if (!newFeed || !popularFeed)
    return (
      <main className="mx-auto flex flex-row w-full max-w-6xl">
        <span>Unable to load feeds</span>
      </main>
    )

  const feeds = [
    {
      feedMeta: newFeed,
      posts: newFeedPosts,
    },
    {
      feedMeta: popularFeed,
      posts: popularFeedPosts,
    },
  ]

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Feed feeds={feeds} />
    </main>
  )
}
