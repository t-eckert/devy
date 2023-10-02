import Shoulder from "@/components/segments/Shoulder"
import Changelog from "@/components/segments/Changelog"
import Feeds, { Content, fetchContent } from "@/lib/feed"
import { fetchChangelog } from "@/lib/changelog"

export default async function Home() {
  const feeds = await fetchFeeds(0, 15)
  const changelog = await fetchChangelog()

  return (
    <main className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <Feeds content={feeds} defaultSelected={"new"} />
      <Shoulder>{changelog && <Changelog changelog={changelog} />}</Shoulder>
    </main>
  )
}

const fetchFeeds = async (
  page: number,
  pageSize: number
): Promise<Content[]> => {
  const feeds: Content[] = []

  const newContent = await fetchContent("new", page, pageSize)
  if (newContent) feeds.push(newContent)

  const popularContent = await fetchContent("popular", page, pageSize)
  if (popularContent) feeds.push(popularContent)

  return feeds
}
