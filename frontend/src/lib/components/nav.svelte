<script lang="ts">
	import ProfileToken from "./profile-token.svelte"
	import Menu from "$lib/components/menu"
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
			class="px-2 py-0.5 text-sm font-medium text-stone-600 hover:text-stone-950 hover:bg-stone-100 rounded-full transition"
			>Sign in</a
		>
	{/if}

	<div>
		<Menu>
			<Menu.Link href="/feedback">Feedback</Menu.Link>
			<Menu.Link href="/changelog">Changelog</Menu.Link>
			<Menu.Link href="/about">About</Menu.Link>
			{#if sessionState.isAdmin()}
				<Menu.Link href="/admin">Admin</Menu.Link>
			{/if}

			{#if sessionState.session}
				<Menu.Sep />
				<Menu.FormButton action="/auth/sign-out" name="sign-out">Sign out</Menu.FormButton>
			{/if}
		</Menu>
	</div>
</nav>
