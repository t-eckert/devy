<script lang="ts">
	import type { Feed } from "$lib/types"
	import FeedEntries from "./feed-entries.svelte"

	export let feed: Feed
</script>

<div class="mx-auto w-full max-w-xl">
	<FeedEntries entries={feed.entries} />
</div>
