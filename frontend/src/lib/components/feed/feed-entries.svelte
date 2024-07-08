<script lang="ts">
	import type { Entry } from '$lib/types';
	import Column from '$lib/layouts/column.svelte';
	import EntryPreview from '$lib/components/entry-preview';

	export let entries: Entry[];
</script>

<Column>
	{#each entries as { title, blogName, blogSlug, postSlug, createdAt, authorName, authorSlug }}
		<EntryPreview>
			<EntryPreview.Blog slot="blog" {blogName} {blogSlug} />
			<EntryPreview.Title slot="title" {title} {blogSlug} {postSlug} />
			<EntryPreview.Author slot="author" {authorName} {authorSlug} />
			<EntryPreview.Date slot="date" date={createdAt} />
		</EntryPreview>
	{/each}
</Column>
