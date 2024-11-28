<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<main class="h-full min-h-screen w-full">
	{@render children()}
</main>
