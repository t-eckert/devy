<script lang="ts">
	import type { PageData } from "./$types"
	import Main from "$lib/layouts/main.svelte"
	import ProfilePreview from "$lib/components/profile-preview.svelte"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview/"

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
					<EntryPreview>
						<EntryPreview.Blog slot="blog" {blogName} {blogSlug} />
						<EntryPreview.Title slot="title" {title} {blogSlug} {postSlug} />
						<EntryPreview.Date slot="date" date={createdAt} />
					</EntryPreview>
				{/each}
			</Column>
		</div>
	</div>
</Main>
