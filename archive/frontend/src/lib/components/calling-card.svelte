<script lang="ts">
	import type { Blog, Profile, User } from "$lib/types"

	export let profile: Profile
	export let blogs: Blog[]
	export let user: User

	let { displayName, avatarUrl, createdAt, bio } = profile
</script>

<div class="rounded-2xl w-56 h-64 flex flex-col shadow select-none">
	<img src={avatarUrl} alt={displayName} class="rounded-2xl w-56 h-64 object-cover" />

	<div class="z-10 absolute h-64 w-56 flex flex-col justify-end rounded-2xl">
		<div
			class="backdrop-blur-sm bg-gradient-to-t from-stone-900/80 to-transparent flex flex-col items-start rounded-b-2xl p-2"
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
