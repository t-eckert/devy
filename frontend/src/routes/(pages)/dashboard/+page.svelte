<script lang="ts">
	import Json from "$lib/utils/json.svelte"
	import Entry from "$lib/components/entry-preview.svelte"

	const { data } = $props()
	const { drafts } = data
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<h1 class="text-xl font-semibold mb-4">Dashboard</h1>

<section>
	<h1 class="font-semibold text-stone-700">Drafts</h1>
	<div class="w-full min-h-56 rounded-md border border-stone-200 bg-stone-100">
		{#if drafts.length === 0}
			<div class="w-full h-full min-h-56 flex items-center justify-center">
				<span class="text-stone-700">No drafts</span>
			</div>
		{:else}
			<div class="p-3 flex flex-row flex-wrap gap-3">
				{#each drafts as entry}
					<Entry {entry} />
				{/each}
			</div>
		{/if}
	</div>
</section>
