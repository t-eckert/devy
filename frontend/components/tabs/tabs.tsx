import { Dispatch, SetStateAction } from "react"

import Tab from "./tab"

interface Props {
	labels: { id: string; name: string }[]
	selected: string
	setSelected: Dispatch<SetStateAction<string>>
}

export default function Tabs({ labels, selected, setSelected }: Props) {
	return (
		<section className="flex flex-row md:w-44 md:flex-col items-state gap-2">
			{labels.map((label) => (
				<Tab
					key={label.id}
					id={label.id}
					isSelected={label.id === selected}
					setSelected={setSelected}
				>
					{label.name}
				</Tab>
			))}
		</section>
	)
}
