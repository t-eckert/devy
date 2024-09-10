<script lang="ts">
	import { useClerkContext } from "svelte-clerk"

	const ctx = useClerkContext()
	const userId = $derived(ctx.auth.userId)
</script>

{#if userId === undefined}
	<p>Loading...</p>
{:else if userId === null}
	<p>Sign in to view this page</p>
{:else}
	<div>{userId}</div>
{/if}
