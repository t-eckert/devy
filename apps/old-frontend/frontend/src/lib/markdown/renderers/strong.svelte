<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<strong {...props} class="font-medium text-zinc-900">{@render children()}</strong>
