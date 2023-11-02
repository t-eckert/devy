import Changelog from "@/components/changelog"
import Shoulder from "./shoulder"

import { fetchChangelog } from "@/lib/changelog"
import fetchFeed from "@/lib/feed"

import HomeFeed from "./home-feed"

export default async function Home() {
  const feeds = await Promise.all([
    fetchFeed("new", 0, 15),
    fetchFeed("popular", 0, 15),
  ])

  const changelog = await fetchChangelog()

  return (
    <section className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HomeFeed feeds={feeds} defaultSelected={"new"} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </section>
  )
}
