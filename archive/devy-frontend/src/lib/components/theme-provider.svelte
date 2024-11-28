<script lang="ts">
	import type { Snippet } from "svelte"
	import { getTheme } from "$lib/state/theme.svelte"

	let { children }: { children: Snippet } = $props()

	let currentTheme = getTheme()
</script>

<div class={currentTheme.theme}>
	<div class="bg-stone-50 text-stone-950 dark:bg-zinc-900 dark:text-white">
		{@render children()}
	</div>
</div>
