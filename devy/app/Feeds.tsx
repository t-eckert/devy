"use client"

import { useState } from "react"

import Sidebar from "@/components/sections/Sidebar"
import FeedSection from "@/components/sections/Feed"

import Tab from "@/models/Tab"
import Feed from "@/models/Feed"

interface Props {
  tabs: Tab[]
  feeds: Feed[]
}

export default function Feeds({ tabs, feeds }: Props) {
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
