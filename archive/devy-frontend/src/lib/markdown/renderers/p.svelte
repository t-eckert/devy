<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<p {...props} class="mb-4 leading-snug text-zinc-900">
	{@render children()}
</p>
