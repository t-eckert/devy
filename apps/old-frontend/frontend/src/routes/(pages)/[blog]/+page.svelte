<script lang="ts">
	import Header from "./header.svelte"
	import Entries from "./entries.svelte"

	const { data } = $props()
	const { blog, entries, isUserFollowing } = data
</script>

<svelte:head>
	<title>{blog.name}</title>
</svelte:head>

<Header {blog} {isUserFollowing} />
<div class="w-full max-w-md">
	<Entries {entries} />
</div>
