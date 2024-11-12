<script lang="ts">
	import { getSession } from "$lib/state/session.svelte"
	import Button from "$lib/components/button.svelte"
	import ProfileToken from "$lib/components/profile-token.svelte"

	let session = getSession()
</script>

{#if session.isAuthenticated}
	<ProfileToken
		username={session.username || "Stranger"}
		displayName={session.displayName}
		avatarUrl={session.avatarUrl}
	/>
{:else}
	<Button role="tertiary" href="/auth/sign-in"
		><span class="text-sm font-medium">Sign in</span></Button
	>
{/if}
