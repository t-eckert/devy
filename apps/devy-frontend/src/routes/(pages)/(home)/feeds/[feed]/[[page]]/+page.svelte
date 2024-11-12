<script lang="ts">
	import EmptyState from "$lib/components/empty-state.svelte"
	import OgHeader from "$lib/components/og-header.svelte"
	import Entry from "$lib/components/entry.svelte"
	import Column from "$lib/layouts/column.svelte"

	let { data } = $props()
	let { feed } = $derived(data)
</script>

<svelte:head>
	<OgHeader title="Devy/Feeds" />
</svelte:head>

<div class="mb-1 flex flex-row justify-between">
	<h1 class="text-sm font-medium text-stone-800">{feed.config.name}</h1>
</div>

{#if feed.entries.length === 0}
	<EmptyState message="No posts found." />
{:else}
	<Column>
		{#each feed.entries as entry}
			<Entry {entry} />
		{/each}
	</Column>
{/if}
