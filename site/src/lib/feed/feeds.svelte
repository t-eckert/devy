<script lang="ts">
	import type { Feed } from "$lib/types"
	import Column from "$lib/layouts/column.svelte"
	import Preview from "$lib/entry/preview/preview.svelte"
	import Title from "$lib/entry/preview/segments/title.svelte"
	import Date from "$lib/entry/preview/segments/date.svelte"
	import Author from "$lib/entry/preview/segments/author.svelte"
	import Blog from "$lib/entry/preview/segments/blog.svelte"

	export let feeds: Feed[]

	let id = "recent"
</script>

<section class="grid grid-cols-1 sm:grid-cols-3 gap-6 sm:gap-8">
	<button-group class="col-start-1 flex flex-row sm:flex-col items-start gap-2">
		{#each feeds as feed}
			<button
				on:click={() => (id = feed.feedConfig.id)}
				class={[
					"bg-zinc-50 text-sm  px-2 py-1 sm:w-52 flex flex-row items-start font-medium rounded-lg border hover:pl-4 transition-all",
					id === feed.feedConfig.id
						? "text-zinc-900 border-zinc-200 shadow"
						: "text-zinc-600 border-zinc-50"
				].join(" ")}>{feed.feedConfig.name}</button
			>
		{/each}
	</button-group>

	<div class="sm:col-start-2 sm:col-span-2">
		{#each feeds as feed}
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
		{/each}
	</div>
</section>
