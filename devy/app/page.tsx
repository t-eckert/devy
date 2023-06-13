import Feeds from "./Feeds"

import { getFeeds } from "@/models/Feed"

export default function Home() {
  const feeds = getFeeds()

  return (
    <>
      <main className="mx-auto mt-12 px-1 w-full max-w-6xl flex flex-row justify-between gap-4">
        <Feeds feeds={feeds} />
        <section className="w-44">Secondary info</section>
      </main>
      <footer></footer>
    </>
  )
}
