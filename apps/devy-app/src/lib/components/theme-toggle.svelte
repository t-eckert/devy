<script lang="ts">
	import Sun from "$lib/icons/sun.svelte";
	import Moon from "$lib/icons/moon.svelte";
	import Button from "$lib/components/button.svelte";
	import { getTheme } from "$lib/state/theme.svelte";

	let theme = getTheme();

	let props = $props();
</script>

<Button onclick={() => theme.toggle()} role="tertiary"
	><span class="text-sm">
		{#if theme.theme === "light"}
			<Sun />
		{:else}
			<Moon />
		{/if}
	</span></Button
>
