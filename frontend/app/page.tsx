import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query"

import fetchFeed from "@/lib/feed"

import MainFeed from "./main.feed"
import Shoulder from "./shoulder"

export default async function HomePage() {
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
        <MainFeed defaultSelected={"new"} />
      </HydrationBoundary>
      <Shoulder></Shoulder>
    </section>
  )
}
