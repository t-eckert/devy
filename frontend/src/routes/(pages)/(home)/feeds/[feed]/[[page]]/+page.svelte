<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import Feed from "$lib/components/feed.svelte"

	const { data } = $props()
	let feed = $derived(data.feed)
</script>

<svelte:head>
	<title>Devy / {feed.config.name}</title>
</svelte:head>

<Feed {feed} />
