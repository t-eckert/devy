<script lang="ts">
	import { Sun } from 'radix-icons-svelte';
	import Menu from '$lib/menu/menu.svelte';
	import Items from '$lib/menu/items.svelte';
</script>

<Menu>
	<Sun slot="icon" />
	<Items slot="items">
		<a href="/theme-switcher">Theme Switcher</a>
	</Items>
</Menu>
