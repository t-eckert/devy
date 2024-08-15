<script lang="ts">
	import type { Entry } from "$lib/types"
	import Column from "$lib/layouts/column.svelte"
	import EntryPreview from "$lib/components/entry-preview.svelte"

	interface Props {
		entries: Entry[]
	}

	const { entries }: Props = $props()
</script>

<Column>
	{#each entries as entry}
		<EntryPreview {entry} omit={["blog"]} />
	{/each}
</Column>
