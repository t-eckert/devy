<script lang="ts">
	import ProfileToken from "$lib/components/profile-token.svelte"
	import NavigationMenu from "$lib/components/navigation-menu.svelte"
	import { getUser } from "$lib/state/user.svelte"

	const user = getUser()
</script>

<nav class="flex flex-row items-center gap-1">
	{#if user.isAuthenticated}
		<ProfileToken
			username={user.username || "Stranger"}
			displayName={user.displayName}
			avatarUrl={user.avatarUrl}
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
