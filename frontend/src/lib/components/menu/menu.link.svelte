<script lang="ts">
	import MenuItem from "./menu.item.svelte"

	let { children, href } = $props()
</script>

<MenuItem>
	<a {href} class="w-full cursor-pointer">
		{#if children}
			{@render children()}
		{/if}
	</a>
</MenuItem>
