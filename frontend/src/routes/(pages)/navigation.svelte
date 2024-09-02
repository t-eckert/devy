<script lang="ts">
	import GithubLogo from "$lib/icons/github-logo.svelte"
	import ProfileToken from "$lib/components/profile-token.svelte"
	import Menu from "$lib/components/menu"
	import { getSessionState } from "$lib/state/session.svelte"
	import PlusIcon from "$lib/icons/plus.svelte"
	import SearchIcon from "$lib/icons/magnifying-glass.svelte"

	const sessionState = getSessionState()
</script>

<nav class="flex flex-row items-center gap-1">
	<button class="p-1 rounded bg-white border border-stone-100">
		<SearchIcon />
	</button>
	<a href="/editor" class="p-1 rounded bg-white border border-stone-100">
		<PlusIcon />
	</a>

	<Menu>
		{#if sessionState.session}
			<Menu.Link href="/dashboard">Dashboard</Menu.Link>
			<Menu.FormButton action="/auth/sign-out" name="sign-out">Sign out</Menu.FormButton>
		{/if}
	</Menu>
</nav>
