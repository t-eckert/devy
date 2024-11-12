<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<pre
	{...props}
	class="bg-zinc-900 text-zinc-50 p-3 rounded-md overflow-auto mb-2 text-sm">{@render children()}</pre>
