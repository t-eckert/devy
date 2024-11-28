<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h5 class="mb-0 mt-4 font-semibold text-stone-800 dark:text-zinc-200">
	{@render children()}
</h5>
