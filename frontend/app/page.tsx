import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query"

import fetchFeed from "@/lib/feed"

import MainFeed from "./main.feed"
import Shoulder from "./shoulder"

const newId = "5941b29d-246d-4897-a69e-3201f6ad8715"

export default async function HomePage() {
  const queryClient = new QueryClient()

  await queryClient.prefetchQuery({
    queryKey: ["feed", "new"],
    queryFn: () => fetchFeed("new"),
  })

  await queryClient.prefetchQuery({
    queryKey: ["feed", "popular"],
    queryFn: () => fetchFeed("popular"),
  })

  return (
    <section className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HydrationBoundary state={dehydrate(queryClient)}>
        <MainFeed defaultSelected={newId} />
      </HydrationBoundary>
      <Shoulder></Shoulder>
    </section>
  )
}
