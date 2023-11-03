"use client"

import React, { useState } from "react"
import { QueryClientProvider, QueryClient } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

import Tabs from "@/components/tabs"

import { FeedMetadata, Post } from "@/models"
import Feed from "@/components/feed"

interface Props {
  feeds: { feedMetadata: FeedMetadata; posts: Post[] }[]
  defaultSelected: string
}

function HomeFeed({ feeds, defaultSelected }: Props) {
  const [selected, setSelected] = useState<string>(defaultSelected)

  return (
    <section className="w-full flex flex-col md:flex-row items-start gap-4">
      <Tabs
        labels={feeds.map(({ feedMetadata }) => ({
          id: feedMetadata.id,
          name: feedMetadata.name,
        }))}
        selected={selected}
        setSelected={setSelected}
      />
      <div className="flex-1 flex flex-row items-center justify-center">
        <Feed
          feed={feeds.find(({ feedMetadata }) => feedMetadata.id === selected)}
        />
      </div>
    </section>
  )
}

const queryClient = new QueryClient()

const HomeFeedWithProvider = (props: Props) => {
  return (
    <QueryClientProvider client={queryClient}>
      <HomeFeed {...props} />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

export default HomeFeedWithProvider
