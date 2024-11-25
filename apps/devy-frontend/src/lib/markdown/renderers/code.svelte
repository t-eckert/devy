<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<code class="rounded bg-stone-200 px-0.5 font-mono text-[0.9em] text-zinc-950"
	>{@render children()}</code
>
