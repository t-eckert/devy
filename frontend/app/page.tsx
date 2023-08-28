import Feed from "@/components/dynamic/Feed"
import Shoulder from "@/components/segments/Shoulder"
import Changelog from "@/components/segments/Changelog"

import feedController from "@/controllers/feed"
import postController from "@/controllers/post"
import changelogController from "@/controllers/changelog"

export default async function Home() {
  const newFeed = await feedController.get.new()
  const newFeedPosts = (await postController.get.byFeed(newFeed.id)) || []
  const changelog = await changelogController.get.fromGitHub()

  if (!newFeed)
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
  ]

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Feed feeds={feeds} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </main>
  )
}
