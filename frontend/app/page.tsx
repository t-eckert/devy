import { dehydrate, HydrationBoundary } from "@tanstack/react-query"

import Shoulder from "@/components/shoulder"
import Feed from "@/components/feed"

import data from "./data"

export default async function HomePage() {
  const { id, queryClient } = await data()

  return (
    <section className="mx-auto my-4 flex flex-col sm:flex-row justify-between px-2 w-full max-w-6xl gap-4 sm:gap-2">
      <HydrationBoundary state={dehydrate(queryClient)}>
        <Feed defaultSelected={id} />
      </HydrationBoundary>
      <Shoulder />
    </section>
  )
}
