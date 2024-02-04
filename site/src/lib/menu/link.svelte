<script lang="ts">
	import { HamburgerMenu } from 'radix-icons-svelte';
	import { createDropdownMenu, melt } from '@melt-ui/svelte';
	const {
		elements: { menu, item, trigger, arrow }
	} = createDropdownMenu();

	export let href = '#';
</script>

<a {href}><slot /></a>
