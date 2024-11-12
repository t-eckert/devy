<script lang="ts">
	import type { Snippet } from "svelte"

	const { children, ...props }: { children: Snippet } = $props()
</script>

<h2
	{...props}
	class="text-2xl sm:text-3xl font-semibold leading-tight text-stone-800 dark:text-zinc-200 mt-4 mb-3"
>
	{@render children()}
</h2>
