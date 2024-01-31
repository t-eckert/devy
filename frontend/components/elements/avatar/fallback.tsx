import * as RadixAvatar from "@radix-ui/react-avatar"

interface Props {
	name: string
}

export default function Fallback({ name }: Props) {
	return (
		<RadixAvatar.Fallback
			className={[
				"w-6 aspect-square flex items-center justify-center text-xs",
				"bg-neutral-neutral+1 text-neutral-1 dark:bg-neutral-1 dark:text-neutral+1",
				"rounded-full font-semibold shadow-md",
			].join(" ")}
		>
			<span className="m-1 select-none">
				{name
					.split(" ")
					.map((name) => name[0].toUpperCase())
					.slice(0, 2)
					.join("")}
			</span>
		</RadixAvatar.Fallback>
	)
}
