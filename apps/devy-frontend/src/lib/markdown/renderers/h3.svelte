<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h3 class="mb-2 mt-4 text-lg font-semibold text-stone-800 dark:text-zinc-200">
	{@render children()}
</h3>
