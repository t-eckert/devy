<script lang="ts">
	import { createDropdownMenu, melt } from "@melt-ui/svelte"
	const {
		elements: { menu, item, trigger, arrow }
	} = createDropdownMenu()
</script>

<button use:melt={$trigger}>Toggle</button>
<section use:melt={$menu}>
	<div use:melt={$item}><a href="/changelog">Changelog</a></div>
</section>
