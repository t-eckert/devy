<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div class="flex flex-row justify-end gap-1 border-t border-stone-100 p-1 dark:border-zinc-800">
	{@render children()}
</div>
