<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<blockquote class="border-l-2 border-stone-400 pl-4 dark:border-zinc-500">
	{@render children()}
</blockquote>
