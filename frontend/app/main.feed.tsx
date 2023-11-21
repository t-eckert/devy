"use client"

import React, { useState } from "react"
import { useQuery } from "@tanstack/react-query"

import Tabs from "@/components/tabs"
import Feed from "@/components/feed"

import fetchFeed from "@/lib/feed"

interface Props {
  defaultSelected: string
}

function HomeFeed({ defaultSelected }: Props) {
  const { data: newFeed } = useQuery({
    queryKey: ["feed", "new", 0, 25],
    queryFn: () => fetchFeed("new", 0, 25),
  })

  const { data: popularFeed } = useQuery({
    queryKey: ["feed", "popular", 0, 25],
    queryFn: () => fetchFeed("popular", 0, 25),
  })

  const [selected, setSelected] = useState<string>(defaultSelected)

  const feeds = [newFeed, popularFeed]

  return (
    <section className="w-full flex flex-col md:flex-row items-start gap-4">
      <div className="pb-2 border-b border-neutral+2 dark:border-neutral-2 md:border-b-neutral+1 md:dark:border-b-neutral-1">
        <Tabs
          labels={feeds.map((feed) => ({
            id: feed?.feedMetadata.id || "",
            name: feed?.feedMetadata.name || "",
          }))}
          selected={selected}
          setSelected={setSelected}
        />
      </div>
      <div className="flex-1 flex flex-row items-center justify-center">
        <Feed feed={feeds.find((feed) => feed?.feedMetadata.id === selected)} />
      </div>
    </section>
  )
}

export default HomeFeed