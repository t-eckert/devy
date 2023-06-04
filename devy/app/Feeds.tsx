"use client"

// TODO: I'd like to give this component a better name.

import Tab from "@/models/Tab"
import Feed from "@/models/Feed"

interface Props {
  tabs: Tab[]
  feeds: Feed[]
}

export default function Feeds({ tabs, feeds }: Props) {
  return <div>Feeds</div>
}
