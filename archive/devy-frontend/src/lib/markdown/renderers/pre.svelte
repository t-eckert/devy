<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<pre
	{...props}
	class="mb-2 overflow-auto rounded-md bg-zinc-900 p-3 text-sm text-zinc-50">{@render children()}</pre>
