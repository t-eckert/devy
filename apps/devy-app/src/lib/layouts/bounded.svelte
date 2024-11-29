<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children: Snippet } = $props();
</script>

<div class="mx-auto h-full w-full max-w-6xl gap-4 px-2 sm:gap-2">
	{@render children()}
</div>
