import HomeFeed from "@/islands/HomeFeed"
import Shoulder from "@/islands/Shoulder"

import { feedGetter } from "@/models/Feed"

export default async function Home() {
  const feeds = await feedGetter.defaults()

  return (
    <main className="mx-auto mt-12 px-1 w-full max-w-6xl flex flex-row justify-between gap-4">
      <HomeFeed feeds={feeds} />
      <Shoulder />
    </main>
  )
}
