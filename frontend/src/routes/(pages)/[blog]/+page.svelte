<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview"
	import Button from "$lib/components/button.svelte"
	import { getSessionState } from "$lib/state/session.svelte"

	export let data: PageData

	// TODO Add author information to a blog so that I don't have to grab it from the first entry
	let authorKnown = false
	const firstEntry = data.props.entries[0]
	if (firstEntry) {
		authorKnown = true
	}
	const { authorName, authorSlug } = firstEntry
</script>

<svelte:head>
	<title>{data.props.blog.name}</title>
</svelte:head>

<Main>
	<div class="mx-auto w-full max-w-2xl">
		<section class="mt-8 mb-4 flex flex-col gap-2">
			<H1>{data.props.blog.name}</H1>
			<div class="flex flex-row items-baseline justify-between mt-2">
				{#if authorKnown}
					<p class="text-sm">By <a href={`/profiles/${authorSlug}`}>{authorName}</a></p>
				{/if}
				{#if getSessionState().isCurrentUser(data.props.blog.userId)}
					<form method="POST" action="?/delete">
						<Button role="tertiary" behavior="negative"
							><span class="text-sm">Delete blog</span></Button
						>
					</form>
				{/if}
			</div>
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
