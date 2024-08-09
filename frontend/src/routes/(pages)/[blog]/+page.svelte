<script lang="ts">
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview"
	import Button from "$lib/components/button.svelte"
	import Json from "$lib/utils/json.svelte"

	import Header from "./header.svelte"
	import Entries from "./entries.svelte"

	const { data } = $props()
	const { blog, entries } = data
</script>

<svelte:head>
	<title>{blog.name}</title>
</svelte:head>

<Header {blog} />
<Entries {entries} />
