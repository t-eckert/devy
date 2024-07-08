<script lang="ts">
	let { children } = $props()
</script>

<button
	>Button
	{#if children}
		{@render children()}
	{/if}
</button>
