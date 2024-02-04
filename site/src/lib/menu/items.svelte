<script lang="ts">
	import { HamburgerMenu } from 'radix-icons-svelte';
	import { createDropdownMenu, melt } from '@melt-ui/svelte';
	const {
		elements: { menu, item, trigger, arrow }
	} = createDropdownMenu();
</script>

<section use:melt={$menu}>
	<slot />
</section>
