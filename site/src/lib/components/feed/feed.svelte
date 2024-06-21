<script lang="ts">
	import type { Feed } from "$lib/types"
	import Column from "$lib/layouts/column.svelte"
	import Preview from "$lib/components/entry-preview/preview.svelte"
	import Title from "$lib/components/entry-preview/segments/title.svelte"
	import Date from "$lib/components/entry-preview/segments/date.svelte"
	import Author from "$lib/components/entry-preview/segments/author.svelte"
	import Blog from "$lib/components/entry-preview/segments/blog.svelte"

	export let feed: Feed
</script>

<div class="mx-auto flex flex-col gap-4">
	<Column>
		{#each feed.entries as { title, blogName, blogSlug, postSlug, createdAt, authorName, authorSlug }}
			<Preview>
				<Blog slot="blog" {blogName} {blogSlug} />
				<Title slot="title" {title} {blogSlug} {postSlug} />
				<Author slot="author" {authorName} {authorSlug} />
				<Date slot="date" date={createdAt} />
			</Preview>
		{/each}
	</Column>
</div>
