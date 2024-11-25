<script lang="ts">
	import type { Snippet } from "svelte"

	let { children, ...props }: { children: Snippet } = $props()
</script>

<a {...props} class="text-stone-700 underline">{@render children()}</a>
