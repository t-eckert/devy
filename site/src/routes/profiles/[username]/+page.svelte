<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import CallingCard from "$lib/components/calling-card/calling-card.svelte"
	import Preview from "$lib/entry/preview/preview.svelte"
	import Column from "$lib/layouts/column.svelte"
	import Title from "$lib/entry/preview/segments/title.svelte"
	import Date from "$lib/entry/preview/segments/date.svelte"
	import H2 from "$lib/elements/h2.svelte"
	import Blog from "$lib/entry/preview/segments/blog.svelte"
	export let data: PageData

	let { blogs, profile, user, entries } = data.props
</script>

<Main>
	<div class="flex flex-col sm:flex-row gap-8">
		<CallingCard {blogs} {profile} {user} />
		<div class="flex flex-col gap-4">
			<H2>Blog posts</H2>
			<Column>
				{#each entries as { title, blogName, blogSlug, postSlug, createdAt, authorName, authorSlug }}
					<Preview>
						<Blog slot="blog" {blogName} {blogSlug} />
						<Title slot="title" {title} {blogSlug} {postSlug} />
						<Date slot="date" date={createdAt} />
					</Preview>
				{/each}
			</Column>
		</div>
	</div>
</Main>
