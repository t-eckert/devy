<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<s {...props} class="text-zinc-900 decoration-red-500 decoration-wavy">{@render children()}</s>
