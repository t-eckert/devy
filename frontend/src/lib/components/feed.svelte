<script lang="ts">
	import type { Feed } from "$lib/types"
	import EntryList from "$lib/components/entry-list.svelte"

	const { feed }: { feed: Feed } = $props()
</script>

<div class="mx-auto w-full max-w-xl">
	{#if feed.count === 0}
		No posts found.
	{/if}
	<EntryList entries={feed.entries} />
</div>
