<script lang="ts">
	import type { Snippet } from "svelte"

	interface Props {
		actions: Snippet
		bar: Snippet
		body: Snippet
	}

	const { actions, bar, body }: Props = $props()
</script>

<div
	class="rounded border border-stone-200 bg-white shadow-sm dark:border-zinc-700 dark:bg-zinc-950 sm:min-w-96"
>
	{@render bar()}
	{@render body()}
	{@render actions()}
</div>
