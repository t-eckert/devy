<script lang="ts">
	import BlogHeader from "./blog-header.svelte"
	import Entry from "$lib/components/entry.svelte"

	let { data } = $props()
	let { blog, entries, isUserFollowing } = data
</script>

<svelte:head>
	<title>{blog.name}</title>
</svelte:head>

<BlogHeader {blog} {isUserFollowing} />

<section class="grid gap-3 sm:grid-cols-2">
	{#each entries as entry}
		<Entry {entry} />
	{/each}
</section>
