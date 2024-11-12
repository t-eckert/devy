<script lang="ts">
	import type { Entry } from "$lib/types"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview.svelte"

	const { entries }: { entries: Entry[] } = $props()
</script>

<Column>
	{#each entries as entry}
		<EntryPreview {entry} />
	{/each}
</Column>
