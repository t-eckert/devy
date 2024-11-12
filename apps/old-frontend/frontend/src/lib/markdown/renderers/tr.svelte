<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<tr {...props} class="w-full">{@render children()}</tr>
