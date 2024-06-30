<script lang="ts">
	import type { PageData } from "./$types"
	import Feed from "$lib/components/feed/feed.svelte"

	export let data: PageData
</script>

<svelte:head>
	<title>{data.feed.feedConfig.name}</title>
</svelte:head>

<Feed feed={data.feed} />
