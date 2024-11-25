<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<ul {...props} class="mb-4 ml-4 mt-2 list-outside list-disc">{@render children()}</ul>
