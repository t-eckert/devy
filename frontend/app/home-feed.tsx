"use client"

import React, { useState } from "react"
import { QueryClientProvider, QueryClient } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"

import Feed from "@/components/feed"
import Tabs from "@/components/tabs"

import { Feed as IFeed, Post } from "@/models"

interface Content {
  feed: IFeed
  posts: Post[]
}

const queryClient = new QueryClient()

interface Props {
  feeds: any
  defaultSelected: string
}

function Feeds({ feeds, defaultSelected }: Props) {
  const [selected, setSelected] = useState<string>(defaultSelected)

  return (
    <section className="w-full flex flex-col md:flex-row items-start gap-4">
      {/* <Tabs */}
      {/*   feeds={content.map(({ feed }) => feed)} */}
      {/*   selected={selected} */}
      {/*   setSelected={setSelected} */}
      {/* /> */}
      {/* <Feed */}
      {/*   initialContent={content.find((content) => content.feed.id === selected)} */}
      {/* /> */}
    </section>
  )
}

const FeedsWithProvider = (props: Props) => {
  return (
    <QueryClientProvider client={queryClient}>
      <Feeds {...props} />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

export default FeedsWithProvider
