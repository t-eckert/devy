<script lang="ts">
	import Collection from "$lib/components/collection.svelte"
	import OGHeader from "$lib/components/og-header.svelte"

	const { data } = $props()
</script>

<svelte:head>
	<OGHeader title="Devy/Bookmarked: git push blog" />
</svelte:head>

<Collection name="Bookmarked" entries={data.collection.entries} />
