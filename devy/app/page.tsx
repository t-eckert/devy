import Feeds from "./Feeds"

import { getTabs } from "@/models/Tab"
import { getFeeds } from "@/models/Feed"

export default function Home() {
  const tabs = getTabs()
  const feeds = getFeeds()

  return (
    <>
      <main className="mx-auto mt-12 px-1 w-full max-w-6xl flex flex-row justify-between gap-4">
        <Feeds tabs={tabs} feeds={feeds} />
        <section className="w-44">Secondary info</section>
      </main>
      <footer></footer>
    </>
  )
}
