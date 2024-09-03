<script lang="ts">
	import Placeholder from "$lib/components/placeholder.svelte"
	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
</script>

<svelte:head>
	<title>Devy / Liked</title>
</svelte:head>

<div class="w-full h-40 flex items-center justify-center">
	<Placeholder>Liked</Placeholder>
</div>
