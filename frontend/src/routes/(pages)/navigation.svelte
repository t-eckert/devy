<script lang="ts">
	import ProfileToken from "$lib/components/profile-token.svelte"
	import NavigationMenu from "$lib/components/navigation-menu.svelte"
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
			data-sveltekit-preload-data="off"
			href="/auth/sign-in"
			class="px-2 py-0.5 flex flex-row items-center gap-2 text-sm font-medium text-stone-600 hover:text-stone-950 hover:bg-stone-200 rounded-full transition"
		>
			<span>Sign in</span>
		</a>
	{/if}

	<NavigationMenu />
</nav>
