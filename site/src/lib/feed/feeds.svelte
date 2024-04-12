<script lang="ts">
	import type { Feed } from "$lib/types"
	import Collection from "$lib/entry/collection/collection.svelte"

	export let feeds: Feed[]

	let id = "recent"
</script>

<section class="grid sm:grid-cols-3 gap-8">
	<button-group class="col-start-1 flex flex-col items-start gap-2">
		{#each feeds as feed}
			<button
				on:click={() => (id = feed.feedConfig.id)}
				class={[
					"bg-zinc-50 text-sm  px-2 py-1 w-52 flex flex-row items-start font-medium rounded-lg border hover:pl-4 transition-all",
					id === feed.feedConfig.id
						? "text-zinc-900 border-zinc-200 shadow"
						: "text-zinc-600 border-zinc-50"
				].join(" ")}>{feed.feedConfig.name}</button
			>
		{/each}
	</button-group>

	<div class="col-start-2 col-span-2">
		{#each feeds as feed}
			<Collection entries={feed.entries} />
		{/each}
	</div>
</section>
