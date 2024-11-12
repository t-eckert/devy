<script lang="ts">
	import type { Snippet } from "svelte"

	let { children, ...props }: { children: Snippet } = $props()
</script>

<a {...props} class="underline text-stone-700">{@render children()}</a>
