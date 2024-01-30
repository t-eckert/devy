"use client"

import React, { useState } from "react"
import { useQuery } from "@tanstack/react-query"

import Tabs from "@/components/tabs"
import PostCollection from "@/components/post-collection"

import fetchFeed from "@/lib/feed"

interface Props {
  defaultSelected: string
}

function HomeFeed({ defaultSelected }: Props) {
  const { data: newFeed } = useQuery({
    queryKey: ["feed", "new"],
    queryFn: () => fetchFeed("new"),
  })

  const { data: popularFeed } = useQuery({
    queryKey: ["feed", "popular"],
    queryFn: () => fetchFeed("popular"),
  })

  const [selected, setSelected] = useState<string>(defaultSelected)

  const feeds = [newFeed, popularFeed]

  return (
    <section className="w-full flex flex-col md:flex-row items-start gap-4">
      <div className="pb-2 sm:border-b border-neutral+2 dark:border-neutral-2 md:border-b-neutral+1 md:dark:border-b-neutral-1">
        <Tabs
          labels={feeds.map((feed) => ({
            id: feed?.feedConfig.id || "",
            name: feed?.feedConfig.name || "",
          }))}
          selected={selected}
          setSelected={setSelected}
        />
      </div>
      <div className="flex-1 flex flex-row items-center justify-center">
        <PostCollection
          posts={
            feeds.find((feed) => feed?.feedConfig.id === selected)?.posts || []
          }
        />
      </div>
    </section>
  )
}

export default HomeFeed
