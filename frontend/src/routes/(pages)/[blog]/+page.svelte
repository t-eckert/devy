<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview"

	export let data: PageData
</script>

<Main>
	<H1>{data.props.blog.name}</H1>

	<div class="mx-auto w-full max-w-2xl">
		<Column>
			{#each data.props.entries as { title, blogSlug, postSlug, createdAt, authorName, authorSlug }}
				<EntryPreview>
					<EntryPreview.Title slot="title" {title} {blogSlug} {postSlug} />
					<EntryPreview.Author slot="author" {authorName} {authorSlug} />
					<EntryPreview.Date slot="date" date={createdAt} />
				</EntryPreview>
			{/each}
		</Column>
	</div>
</Main>
