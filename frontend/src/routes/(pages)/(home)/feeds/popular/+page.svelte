<script lang="ts">
	import Feed from "$lib/components/feed"

	const { data } = $props()

	const feed = data.feed
</script>

<svelte:head>
	<title>Devy: Popular</title>
</svelte:head>

<Feed {feed} />
