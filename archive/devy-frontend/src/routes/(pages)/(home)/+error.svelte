<script lang="ts">
	import { page } from "$app/stores"
	import EmptyState from "$lib/components/empty-state.svelte"
</script>

{#if $page.error}
	<EmptyState message="Error fetching posts." />
{/if}
