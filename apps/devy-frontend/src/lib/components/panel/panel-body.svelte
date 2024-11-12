<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div class="w-full p-3">{@render children()}</div>
