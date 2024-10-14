<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h1
	class="text-xl sm:text-4xl font-semibold break-normal text-stone-800 dark:text-zinc-200 leading-tight mt-4 mb-4"
>
	{@render children()}
</h1>
