import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query"

import Changelog from "@/components/changelog"

import { fetchChangelog } from "@/lib/changelog"
import fetchFeed from "@/lib/feed"

import HomeFeed from "./home-feed"
import Shoulder from "./shoulder"

export default async function HomePage() {
  const changelog = await fetchChangelog()

  const queryClient = new QueryClient()

  await queryClient.prefetchQuery({
    queryKey: ["feed", "new", 0, 25],
    queryFn: () => fetchFeed("new", 0, 25),
  })

  await queryClient.prefetchQuery({
    queryKey: ["feed", "popular", 0, 25],
    queryFn: () => fetchFeed("popular", 0, 25),
  })

  return (
    <section className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HydrationBoundary state={dehydrate(queryClient)}>
        <HomeFeed defaultSelected={"new"} />
      </HydrationBoundary>
      <Shoulder>
        <Changelog changelog={changelog} />
      </Shoulder>
    </section>
  )
}
