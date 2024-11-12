<script lang="ts">
	import { page } from "$app/stores"
	import Error from "$lib/components/error.svelte"
</script>

{#if $page.error}
	<div class="flex h-[80vh] w-full items-center justify-center">
		<Error {...$page} />
	</div>
{/if}
