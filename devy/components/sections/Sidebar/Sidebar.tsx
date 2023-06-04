import TabElement from "@/components/elements/Tab"

import Tab from "@/models/Tab"

interface Props {
  tabs: Tab[]
  selectedFeed: string
  setSelectedFeed: (slug: string) => void
}

export default function Sidebar({
  tabs,
  selectedFeed,
  setSelectedFeed,
}: Props) {
  return (
    <section className="flex flex-col items-start gap-2">
      {tabs.map((tab) => (
        <TabElement
          key={tab.slug}
          tab={tab}
          isSelected={selectedFeed === tab.slug}
          setSelectedTab={setSelectedFeed}
        />
      ))}
    </section>
  )
}
