<script lang="ts">
	import type { Snippet } from "svelte"

	const { children }: { children: Snippet } = $props()
</script>

<h1
	class="mb-4 mt-4 break-normal text-xl font-semibold leading-tight text-stone-800 dark:text-zinc-200 sm:text-4xl"
>
	{@render children()}
</h1>
