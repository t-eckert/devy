<script lang="ts">
	import type { Feed } from "$lib/types"
	import Preview from "$lib/entry/preview/preview.svelte"

	export let feed: Feed
</script>

<section class="flex flex-col gap-6">
	{#each feed.entries as entry}
		<Preview {entry} />
	{/each}
</section>
