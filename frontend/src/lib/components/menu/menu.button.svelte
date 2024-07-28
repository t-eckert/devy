<script lang="ts">
	import MenuItem from "./menu.item.svelte"

	let { children, onclick } = $props()
</script>

<MenuItem>
	<button class="w-full flex flex-row justify-start" {onclick}>
		{#if children}
			{@render children()}
		{/if}
	</button>
</MenuItem>
