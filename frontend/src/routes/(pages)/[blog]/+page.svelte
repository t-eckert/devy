<script lang="ts">
	import Header from "./header.svelte"
	import Entries from "./entries.svelte"

	const { data } = $props()
	const { blog, entries } = data
</script>

<svelte:head>
	<title>{blog.name}</title>
</svelte:head>

<div class="mx-auto max-w-2xl">
	<Header {blog} />
	<Entries {entries} />
</div>
