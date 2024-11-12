<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<thead {...props} class="w-full border-b border-b-slate-400">{@render children()}</thead>
