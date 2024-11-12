<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<blockquote class="border-l-2 border-stone-400 dark:border-zinc-500 pl-4">
	{@render children()}
</blockquote>
