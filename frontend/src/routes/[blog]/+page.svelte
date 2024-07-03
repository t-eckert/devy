<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"

	import Preview from "$lib/components/entry-preview/preview.svelte"
	import Title from "$lib/components/entry-preview/segments/title.svelte"
	import Date from "$lib/components/entry-preview/segments/date.svelte"
	import Author from "$lib/components/entry-preview/segments/author.svelte"

	export let data: PageData
</script>

<Main>
	<H1>{data.props.blog.name}</H1>

	<div class="mx-auto w-full max-w-2xl">
		<Column>
			{#each data.props.entries as { title, blogSlug, postSlug, createdAt, authorName, authorSlug }}
				<Preview>
					<Title slot="title" {title} {blogSlug} {postSlug} />
					<Author slot="author" {authorName} {authorSlug} />
					<Date slot="date" date={createdAt} />
				</Preview>
			{/each}
		</Column>
	</div>
</Main>
