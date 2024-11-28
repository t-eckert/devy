<script lang="ts">
	import type { Entry } from "$lib/types"
	import EntryList from "$lib/components/entry-list.svelte"

	const { name, entries }: { name: string; entries: Entry[] } = $props()
</script>

<div class="mx-auto w-full max-w-xl">
	<div class="mb-1 flex flex-row justify-between">
		<h1 class="text-sm font-medium text-stone-800 transition-all">{name}</h1>
	</div>
	{#if entries.length === 0}
		<div class="w-full h-44 bg-stone-100 rounded flex items-center justify-center">
			No posts found.
		</div>
	{/if}
	<EntryList {entries} />
</div>
