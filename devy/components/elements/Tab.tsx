import { cva, type VariantProps } from "cva"

import Tab from "@/models/Tab"

interface Props extends VariantProps<typeof styles> {
	tab: Tab
	isSelected: boolean
	setSelectedTab: (slug: string) => void
}

const styles = cva(
	["px-2", "py-0.5", "text-sm", "hover:pl-3", "transition-all", "w-44", "flex", "justify-between", "items-center"], {
	variants: {
		isSelected: {
			true: "text-slate-900 font-medium border border-slate-100 shadow rounded-md",
			false: "text-slate-700",
		},
	},
	defaultVariants: {
		isSelected: false,
	},
})

export default function Tab({ tab, isSelected, setSelectedTab }: Props) {
	return <button className={styles({ isSelected })} onClick={() => setSelectedTab(tab.slug)}>
		<span>{tab.title}</span>
		<span className="text-xs">{tab.count}</span>
	</button>
}
