<script lang="ts">
	import ProfileToken from "./profile-token.svelte"
	import { signInUrl } from "$lib/auth"
	import { getSessionState } from "$lib/state/session.svelte"

	const sessionState = getSessionState()
</script>

<nav class="flex flex-row items-center gap-1">
	{#if sessionState.session}
		<ProfileToken
			username={sessionState.session.username}
			displayName={sessionState.session.displayName}
			avatarUrl={sessionState.session.avatarUrl}
		/>
	{:else}
		<a
			href={signInUrl}
			class="px-2 py-0.5 text-sm font-medium text-stone-600 hover:text-stone-950 hover:bg-stone-100 rounded-full transition"
			>Sign in</a
		>
	{/if}
	<a
		href="/feedback"
		class="px-2 py-0.5 text-sm font-medium text-stone-600 hover:text-stone-950 hover:bg-stone-100 rounded-full transition"
		>Feedback</a
	>
	<a
		href="/changelog"
		class="px-2 py-0.5 text-sm font-medium text-stone-600 hover:text-stone-950 hover:bg-stone-100 rounded-full transition"
		>Changelog</a
	>
</nav>
