import { Dispatch, SetStateAction } from "react"

import Tab from "./Tab"
import { Feed } from "@/models"

interface Props {
	feeds: Feed[]
	selected: string
	setSelected: Dispatch<SetStateAction<string>>
}

export default function Tabs({ feeds, selected, setSelected }: Props) {
	return (
		<section className="flex flex-row md:w-44 md:flex-col items-state gap-2">
			{feeds.map((feed) => (
				<Tab
					key={feed.id}
					id={feed.id}
					isSelected={feed.id === selected}
					setSelected={setSelected}
				>
					{feed.name}
				</Tab>
			))}
		</section>
	)
}
