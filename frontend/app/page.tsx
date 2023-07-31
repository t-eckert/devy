import feed from "@/controllers/feed"
import Feed from "@/components/dynamic/Feed"

export default async function Home() {
  const newFeed = await feed.get.new()
  const popularFeed = await feed.get.popular()

  if (!newFeed || !popularFeed)
    return (
      <main className="mx-auto flex flex-row w-full max-w-6xl">
        <span>Unable to load feeds</span>
      </main>
    )

  const feeds = [newFeed, popularFeed]

  return (
    <main className="mx-auto my-12 flex flex-row px-2 w-full max-w-6xl">
      <Feed feeds={feeds} />
    </main>
  )
}
