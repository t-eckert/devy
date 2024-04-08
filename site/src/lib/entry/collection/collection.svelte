<script lang="ts">
	import type { Entry } from "$lib/types"
	import Preview from "$lib/entry/preview/preview.svelte"

	export let entries: Entry[]
</script>

<section class="flex flex-col gap-6 pt-6">
	{#each entries as entry}
		<Preview {entry} />
	{/each}
</section>
