import { VariantProps, cva } from "cva"
import { Dispatch, InputHTMLAttributes, SetStateAction } from "react"

interface Props extends InputHTMLAttributes<HTMLInputElement> {
	label: string
	value: string
	setValue: Dispatch<SetStateAction<string>>
	variant?: VariantProps<typeof styles>
}

export default function Input({
	label,
	variant,
	className,
	value,
	setValue,
	...props
}: Props) {
	return (
		<span
			className={["flex flex-col items-start gap-0.5", className].join(
				" "
			)}
		>
			<label className="text-sm font-medium">{label}</label>
			<input
				className={styles(variant)}
				value={value}
				onChange={(e) => setValue(e.target.value)}
				{...props}
			></input>
		</span>
	)
}

const styles = cva([
	"w-full",
	"px-1.5",
	"py-0.5",
	"rounded",

	"bg-white",
	"shadow-sm",
	"border-neutral-1",

	"placeholder:text-neutral",

	"dark:bg-neutral-2",
	"dark:text-neutral+2",
	"dark:placeholder:text-neutral",

	"transition-all",
])
