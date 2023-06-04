"use client"

import { useState } from "react"

import TabElement from "@/components/elements/Tab"

import Tab from "@/models/Tab"

interface Props {
  tabs: Tab[]
}

export default function Sidebar(props: Props) {
  const [selectedTab, setSelectedTab] = useState<string>(props.tabs[0].slug)

  return (
    <section className="flex flex-col items-start gap-2">
      {props.tabs.map((tab) => (
        <TabElement
          key={tab.slug}
          tab={tab}
          isSelected={selectedTab === tab.slug}
          setSelectedTab={setSelectedTab}
        />
      ))}
    </section>
  )
}
