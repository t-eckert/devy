"use client"

import { useState } from "react"

import Sidebar from "@/components/sections/Sidebar"
import FeedSection from "@/components/sections/Feed"

import Feed from "@/models/Feed"

interface Props {
  feeds: Feed[]
}

export default function Feeds({ feeds }: Props) {
  const tabs = [
    {
      slug: "popular",
      title: "Popular",
    },
    {
      slug: "new",
      title: "New",
    },
  ]
  const [selectedFeed, setSelectedFeed] = useState(tabs[0].slug)

  return (
    <>
      <Sidebar
        tabs={tabs}
        selectedFeed={selectedFeed}
        setSelectedFeed={setSelectedFeed}
      />
      <FeedSection feed={feeds.find((feed) => feed.slug === selectedFeed)} />
    </>
  )
}
