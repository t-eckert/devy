<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<h2
	{...props}
	class="mb-3 mt-4 text-2xl font-semibold leading-tight text-stone-800 dark:text-zinc-200 sm:text-3xl"
>
	{@render children()}
</h2>
