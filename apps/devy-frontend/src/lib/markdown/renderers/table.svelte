<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<div class="w-full rounded bg-white px-3 py-1 shadow">
	<table {...props} class="w-full">{@render children()}</table>
</div>
