"use client"

import * as RadixAvatar from "@radix-ui/react-avatar"

import Fallback from "./fallback"

interface Props {
	name: string
	avatarUrl?: string
}

export default function Avatar({ name, avatarUrl }: Props) {
	return (
		<RadixAvatar.Root>
			{avatarUrl && (
				<RadixAvatar.Image
					className="w-6 aspect-square object-cover rounded-full shadow-md"
					src={avatarUrl}
					alt="Avatar"
				/>
			)}
			<Fallback name={name} />
		</RadixAvatar.Root>
	)
}
