<script lang="ts">
	import type { Snippet } from "svelte"

	const { title, children }: { title?: string; children: Snippet } = $props()
</script>

<div class="mb-3 flex flex-col gap-1">
	{#if title}
		<h1 class="sr-only text-sm font-semibold text-stone-700 dark:text-zinc-200 sm:not-sr-only">
			{title}
		</h1>
	{/if}
	<div class="flex flex-row gap-1 sm:flex-col">
		{@render children()}
	</div>
</div>
