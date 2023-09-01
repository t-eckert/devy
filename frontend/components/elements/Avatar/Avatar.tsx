"use client"

import * as RadixAvatar from "@radix-ui/react-avatar"

interface Props {
	displayName: string
	avatarUrl?: string
}

const initials = (displayName: string) => displayName.split(" ").map(name => name[0]).join("")

export default function Avatar({ displayName, avatarUrl }: Props) {
	return <RadixAvatar.Root className="bg-zinc-700 h-10 w-10 rounded-full">
		{avatarUrl && <RadixAvatar.Image className="object-cover rounded-[inherit]" src={avatarUrl} alt="Avatar" />}
		<RadixAvatar.Fallback className="w-full h-full flex items-center justify-center">
			<div className="font-semibold">
				{initials(displayName)}
			</div>
		</RadixAvatar.Fallback>
	</RadixAvatar.Root>
}

