import { cva, type VariantProps } from "cva"

interface Props extends VariantProps<typeof styles> {
	children: React.ReactNode
	id: number
	isSelected: boolean
	setSelected: (id: number) => void
}


export default function Tab({ children, id, isSelected, setSelected }: Props) {
	return <button className={styles({ isSelected })} onClick={() => setSelected(id)}>
		{children}
	</button>
}


const styles = cva(
	["px-2", "py-0.5", "text-sm", "hover:pl-3", "transition-all", "w-44", "flex", "justify-between", "items-center"], {
	variants: {
		isSelected: {
			true: "text-slate-900 font-medium border border-slate-100 shadow rounded-md",
			false: "text-slate-700 border border-transparent",
		},
	},
	defaultVariants: {
		isSelected: false,
	},
})
