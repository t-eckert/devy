<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<code class="text-[0.9em] px-0.5 bg-stone-200 rounded text-zinc-950 font-mono"
	>{@render children()}</code
>
