<script>
	import { page } from "$app/stores"
	import Error from "$lib/components/error.svelte"
</script>

{#if $page.error}
	<Error {...$page} />
{/if}
