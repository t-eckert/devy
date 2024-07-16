<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview"
	import Json from "$lib/utils/json.svelte"

	export let data: PageData

	// TODO Add author information to a blog so that I don't have to grab it from the first entry
	let authorKnown = false
	const firstEntry = data.props.entries[0]
	if (firstEntry) {
		authorKnown = true
	}
	const { authorName, authorSlug } = firstEntry
</script>

<Main>
	<div class="mx-auto w-full max-w-2xl">
		<section class="mt-8 mb-4 flex flex-col gap-2">
			<H1>{data.props.blog.name}</H1>
			{#if authorKnown}
				<p>By <a href={`/profiles/${authorSlug}`}>{authorName}</a></p>
			{/if}
		</section>
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
