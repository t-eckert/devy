<script lang="ts">
	import Feed from "$lib/components/feed"
	import Placeholder from "$lib/components/placeholder.svelte"
	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
</script>

<svelte:head>
	<title>Devy: Bookmarked</title>
</svelte:head>

<Json {data} />
