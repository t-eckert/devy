<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h4 class="text-lg font-semibold mt-4 mb-0 text-stone-800 dark:text-zinc-200">
	{@render children()}
</h4>
