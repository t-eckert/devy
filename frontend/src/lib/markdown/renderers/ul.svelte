<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<ul {...props} class="ml-4 mt-2 mb-4 list-disc list-outside">{@render children()}</ul>
