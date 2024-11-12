<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h3 class="text-lg font-semibold mt-4 mb-2 text-stone-800 dark:text-zinc-200">
	{@render children()}
</h3>
