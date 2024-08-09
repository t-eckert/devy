<script lang="ts">
	import type { Profile, User } from "$lib/types"
	import Avatar from "./avatar.svelte"

	export let profile: Profile
	export let user: User

	let { displayName, avatarUrl, bio } = profile
</script>

<div class="w-full max-w-56 rounded-xl aspect-square flex flex-col shadow select-none relative">
	<div class="aspect-square rounded-xl">
		<Avatar src={avatarUrl} {displayName} />
	</div>

	<div class="z-10 absolute bottom-0 w-full aspect-square flex flex-col justify-end rounded-xl">
		<div
			class="backdrop-blur-lg bg-gradient-to-t from-stone-900/80 to-transparent flex flex-col items-start rounded-b-xl px-2 pb-2 pt-0.5"
		>
			<div class="text-stone-100 font-medium">
				{displayName || user.username}
			</div>
			{#if bio}
				<p class="-mt-0.75 text-xs text-stone-200">{bio}</p>
			{/if}
		</div>
	</div>
</div>
