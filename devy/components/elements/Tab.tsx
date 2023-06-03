import { cva, type VariantProps } from "cva"

import Tab from "@/models/Tab"

interface Props extends VariantProps<typeof styles> {
	tab: Tab
	isSelected: boolean
	setSelectedTab: (slug: string) => void
}

const styles = cva(
	[], {
	variants: {
		isSelected: {
			true: "bg-gray-100",
			false: "hover:bg-gray-100",
		},
	},
	defaultVariants: {
		isSelected: false,
	},
})

export default function Tab({ tab, isSelected, setSelectedTab }: Props) {
	return <button className={styles()} onClick={() => setSelectedTab(tab.slug)}>{tab.title}</button>
}
