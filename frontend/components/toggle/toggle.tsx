import * as Switch from "@radix-ui/react-switch"

interface Props {
	id: string
	defaultChecked: boolean
	checked: boolean
	onCheckedChange: (checked: boolean) => void
	disabled: boolean
	required: boolean
	name: string
	value: string
}

export default function Toggle({
	id,
	defaultChecked,
	checked,
	onCheckedChange,
	disabled,
	required,
	name,
	value,
}: Props) {
	return (
		<Switch.Root
			id={id}
			defaultChecked={defaultChecked}
			checked={checked}
			onCheckedChange={onCheckedChange}
			disabled={disabled}
			required={required}
			name={name}
			value={value}
			className={root}
		>
			<Switch.Thumb className={thumb} />
		</Switch.Root>
	)
}

const root = [
	"w-8",
	"h-5",

	"bg-neutral+1",
	"dark:bg-neutral-1",
	"disabled:bg-neutral+1",
	"disabled:dark:bg-neutral-1",

	"rounded-full",
	"relative",
	"data-[state=checked]:bg-neutral-1",
	"data-[state=checked]:dark:bg-neutral+1",
	"outline-none",

	"cursor-pointer",
	"disabled:cursor-not-allowed",

	"transition-color",
].join(" ")
const thumb = [
	"block",
	"w-4",
	"aspect-square",

	"bg-neutral+3",
	"dark:bg-neutral-3",

	"data-[disabled]:bg-neutral+2",
	"data-[disabled]:dark:bg-neutral-2",

	"shadow",
	"rounded-full",
	"transition-transform",
	"duration-100",
	"translate-x-0.5",
	"will-change-transform",
	"data-[state=checked]:translate-x-3.5",
].join(" ")
