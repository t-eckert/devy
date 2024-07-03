<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import ProfilePreview from "$lib/components/profile-preview/profile-preview.svelte"
	import Column from "$lib/layouts/column.svelte"
	import Preview from "$lib/components/entry-preview/preview.svelte"
	import Title from "$lib/entry/preview/segments/title.svelte"
	import Date from "$lib/entry/preview/segments/date.svelte"
	import Blog from "$lib/entry/preview/segments/blog.svelte"
	export let data: PageData

	let { profile, user, entries } = data.props
</script>

<Main>
	<div class="flex flex-col sm:flex-row gap-8">
		<div class="mx-auto sm:mx-0">
			<ProfilePreview {profile} {user} />
		</div>

		<div class="flex flex-col gap-4">
			<h2 class="text-2xl sm:text-4xl font-semibold leading-tight text-zinc-900">Blog Posts</h2>
			<Column>
				{#each entries as { title, blogName, blogSlug, postSlug, createdAt }}
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
