<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import H1 from "$lib/elements/h1.svelte"
	import Column from "$lib/layouts/column.svelte"
	import Preview from "$lib/entry/preview/preview.svelte"
	import Title from "$lib/entry/preview/segments/title.svelte"
	import Date from "$lib/entry/preview/segments/date.svelte"
	import Author from "$lib/entry/preview/segments/author.svelte"
	import Blog from "$lib/entry/preview/segments/blog.svelte"

	export let data: PageData
</script>

<Main>
	<div class="flex flex-col gap-4">
		<H1>{data.props.blog.name}</H1>

		<div class="col-start-2 col-span-2">
			<Column>
				{#each data.props.entries as { title, blogName, blogSlug, postSlug, createdAt, authorName, authorSlug }}
					<Preview>
						<Title slot="title" {title} {blogSlug} {postSlug} />
						<Author slot="author" {authorName} {authorSlug} />
						<Date slot="date" date={createdAt} />
					</Preview>
				{/each}
			</Column>
		</div>
	</div>
</Main>
