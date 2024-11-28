<script lang="ts">
	import Feed from "$lib/components/feed.svelte"
	import OGHeader from "$lib/components/og-header.svelte"

	const { data } = $props()
	let feed = $derived(data.feed)
</script>

<svelte:head>
	<OGHeader title="Devy/{feed.config.name}: git push blog" />
</svelte:head>

<Feed {feed} />
