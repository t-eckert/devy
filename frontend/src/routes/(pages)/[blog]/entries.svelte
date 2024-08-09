<script lang="ts">
	import type { Entry } from "$lib/types"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview"

	interface Props {
		entries: Entry[]
	}

	const { entries }: Props = $props()
</script>

<Column>
	{#each entries as { title, blogSlug, postSlug, createdAt, authorName, authorSlug, likes, id: postId }}
		<EntryPreview>
			<EntryPreview.Likes slot="likes" {postId} {likes} />
			<EntryPreview.Title slot="title" {title} {blogSlug} {postSlug} />
			<EntryPreview.Author slot="author" {authorName} {authorSlug} />
			<EntryPreview.Date slot="date" date={createdAt} />
		</EntryPreview>
	{/each}
</Column>
