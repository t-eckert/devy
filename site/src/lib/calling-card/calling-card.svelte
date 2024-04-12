<script lang="ts">
	import type { Profile } from "$lib/types"
	import Json from "$lib/utils/json.svelte"

	export let profile: Profile
	let { displayName, avatarUrl, createdAt } = profile
	let joined = (() => {
		let created = new Date(createdAt)
		return `${created.getDate()}.${created.getMonth()}.${created.getFullYear()}`
	})()
</script>

<div class="rounded w-44 flex flex-col">
	<img src={avatarUrl} alt={displayName} class="rounded-t-lg" />
	<div class="border-b border-x rounded-b-lg border-zinc-200 px-1 py-1 flex flex-col gap-4">
		<span class="text-sm font-medium">{displayName}</span>

		<div class="w-full flex flex-row justify-start">
			<div class="font-mono text-xs px-1 py-0.5 text-zinc-800 rounded">
				{joined}
			</div>
		</div>
	</div>
</div>
