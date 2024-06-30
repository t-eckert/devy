<script lang="ts">
	import type { Feed } from "$lib/types"
	import FeedEntries from "./feed-entries.svelte"

	export let feed: Feed
</script>

<div class="mx-auto w-full max-w-xl">
	<div class="flex flex-row justify-between mb-2">
		<h1 class="font-semibold">{feed.feedConfig.name}</h1>
	</div>
	<FeedEntries entries={feed.entries} />
</div>
