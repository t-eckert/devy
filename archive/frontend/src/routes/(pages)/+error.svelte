<script>
	import { page } from "$app/stores"
	import Error from "$lib/components/error.svelte"
</script>

<div class="h-[80vh] flex items-center justify-center">
	{#if $page.error}
		<Error {...$page} />
	{/if}
</div>
