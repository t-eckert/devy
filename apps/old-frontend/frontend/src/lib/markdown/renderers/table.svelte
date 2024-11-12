<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<div class="w-full bg-white rounded shadow py-1 px-3">
	<table {...props} class="w-full">{@render children()}</table>
</div>
