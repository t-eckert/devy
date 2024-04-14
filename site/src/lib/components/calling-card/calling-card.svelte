<script lang="ts">
	import type { Blog, Profile, User } from "$lib/types"
	import Json from "$lib/utils/json.svelte"

	export let profile: Profile
	export let blogs: Blog[]
	export let user: User

	let { displayName, avatarUrl, createdAt, bio } = profile
	let joined = (() => {
		let created = new Date(createdAt)
		return `${created.getFullYear()}.${created.getMonth()}.${created.getDate()}`
	})()
</script>

<div class="rounded w-56 flex flex-col shadow">
	<img src={avatarUrl} alt={displayName} class="rounded-t" />

	<div class="rounded-b px-1 pt-2 pb-1 flex flex-col gap-4">
		<div>
			<a href={`/profiles/` + user.username} class="text-sm font-medium">{displayName}</a>
			{#if bio}
				<p class="-mt-0.75 text-xs text-zinc-500">{bio}</p>
			{/if}
		</div>

		<div class="flex flex-col">
			{#each blogs as blog}
				<a href={`/` + blog.slug} class="text-xs font-medium text-zinc-700 hover:text-zinc-950"
					>{blog.name}</a
				>
			{/each}
		</div>

		<div class="w-full flex flex-row justify-end">
			<div class="font-mono text-xs px-1 py-0.5 text-zinc-50 bg-zinc-900 rounded">
				{joined}
			</div>
		</div>
	</div>
</div>
