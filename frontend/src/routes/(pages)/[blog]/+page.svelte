<script lang="ts">
	import Header from "./header.svelte"
	import Entries from "./entries.svelte"
	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
	const { blog, entries, isUserFollowing } = data
</script>

<svelte:head>
	<title>{blog.name}</title>
</svelte:head>

<div class="mx-auto max-w-2xl">
	<Header {blog} {isUserFollowing} />
	<Entries {entries} />
</div>
