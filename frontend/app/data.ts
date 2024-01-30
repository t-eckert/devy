import { QueryClient } from "@tanstack/react-query"

import fetchFeed from "@/lib/feed"

const newId = "5941b29d-246d-4897-a69e-3201f6ad8715"

interface Data {
  id: string
  queryClient: QueryClient
}

export default async function data(): Promise<Data> {
  const queryClient = new QueryClient()

  await queryClient.prefetchQuery({
    queryKey: ["feed", "new"],
    queryFn: () => fetchFeed("new"),
  })

  await queryClient.prefetchQuery({
    queryKey: ["feed", "popular"],
    queryFn: () => fetchFeed("popular"),
  })

  return {
    id: newId,
    queryClient,
  }
}
