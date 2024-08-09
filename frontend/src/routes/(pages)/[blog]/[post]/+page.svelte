<script lang="ts">
	import Header from "./header.svelte"
	import Body from "./body.svelte"
	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
	const { entry } = data
</script>

<svelte:head>
	<title>{entry.title}</title>
</svelte:head>

<Header {entry} />
<Body {entry} />
