"use client"

import * as RadixAvatar from "@radix-ui/react-avatar"

interface Props {
	name: string
	avatarUrl?: string
}

export default function Avatar({ name, avatarUrl }: Props) {
	return (
		<RadixAvatar.Root>
			{avatarUrl && (
				<RadixAvatar.Image
					className="w-4 aspect-square object-cover rounded-full shadow-md"
					src={avatarUrl}
					alt="Avatar"
				/>
			)}
			<RadixAvatar.Fallback className={fallback}>
				<span className="m-1 select-none">
					{name
						.split(" ")
						.map((name) => name[0].toUpperCase())
						.slice(0, 2)
						.join("")}
				</span>
			</RadixAvatar.Fallback>
		</RadixAvatar.Root>
	)
}

const fallback = [
	"w-6",
	"aspect-square",

	"flex",
	"items-center",
	"justify-center",

	"text-xs",

	"bg-neutral-neutral+1",
	"text-neutral-1",
	"dark:bg-neutral-1",
	"dark:text-neutral+1",

	"rounded-full",
	"font-semibold",
	"shadow-md",
].join(" ")
