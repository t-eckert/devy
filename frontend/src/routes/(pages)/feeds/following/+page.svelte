<script lang="ts">
	import Feed from "$lib/components/feed"

	const { data } = $props()

	const feed = data.props.feed
</script>

<svelte:head>
	<title>Devy: Following</title>
</svelte:head>

<Feed {feed} />
