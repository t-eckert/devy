<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div class="flex h-[80vh] items-center justify-center">
	{@render children()}
</div>
