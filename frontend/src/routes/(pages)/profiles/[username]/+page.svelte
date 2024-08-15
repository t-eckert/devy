<script lang="ts">
	import ProfilePreview from "$lib/components/profile-preview.svelte"
	import Column from "$lib/layouts/column.svelte"
	import Entry from "$lib/components/entry-preview.svelte"

	import Json from "$lib/utils/json.svelte"

	const { data } = $props()
	const { profile, user, blogs, entries } = data
</script>

<svelte:head>
	<title>{profile.displayName}</title>
</svelte:head>

<div class="flex flex-col gap-3 mx-auto items-center">
	<ProfilePreview {profile} {user} />

	<Column>
		{#each entries as entry}
			<Entry {entry} omit={["author"]} />
		{/each}
	</Column>
</div>
