<script lang="ts">
	import type { PageData } from "./$types"
	import Column from "$lib/layouts/column.svelte"
	import Preview from "$lib/components/entry-preview/preview.svelte"
	import Title from "$lib/entry/preview/segments/title.svelte"
	import Date from "$lib/entry/preview/segments/date.svelte"
	import Author from "$lib/entry/preview/segments/author.svelte"
	import Blog from "$lib/entry/preview/segments/blog.svelte"

	export let data: PageData
</script>

<svelte:head>
	<title>Feed</title>
</svelte:head>

<Column>
	{#each data.feed.entries as { title, blogName, blogSlug, postSlug, createdAt, authorName, authorSlug }}
		<Preview>
			<Blog slot="blog" {blogName} {blogSlug} />
			<Title slot="title" {title} {blogSlug} {postSlug} />
			<Author slot="author" {authorName} {authorSlug} />
			<Date slot="date" date={createdAt} />
		</Preview>
	{/each}
</Column>
