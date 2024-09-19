<script lang="ts">
	import type { Snippet } from "svelte"

	const { title, children }: { title: string; children: Snippet } = $props()
</script>

<div class="flex flex-col gap-1">
	<h1 class="font-semibold text-stone-700 text-sm sr-only sm:not-sr-only dark:text-zinc-200">
		{title}
	</h1>
	<div class="flex flex-row sm:flex-col gap-1">
		{@render children()}
	</div>
</div>
