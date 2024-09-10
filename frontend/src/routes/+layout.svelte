<script lang="ts">
	import "../app.css"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import { onMount } from "svelte"
	import Debug from "$lib/debug/debug.svelte"
	import { dev } from "$app/environment"

	import type { Snippet } from "svelte"
	import type { LayoutData } from "./$types"
	import { ClerkProvider, SignIn } from "svelte-clerk"
	import { PUBLIC_CLERK_PUBLISHABLE_KEY } from "$env/static/public"

	import { setSessionState } from "$lib/state/session.svelte"
	import { setLikesState } from "$lib/state/likes.svelte"
	import { setThemeState } from "$lib/state/theme.svelte"
	import { setUser } from "$lib/state/user.svelte"

	const { children, data } = $props()

	setUser(data.token)

	const themeState = setThemeState("light")

	const sessionState = setSessionState()
	if (data.token) {
		sessionState.setToken(data.token)
	}

	const likesState = setLikesState()
	if (data.token) {
		likesState.setToken(data.token)
	}

	onMount(async () => {
		await likesState.loadLikes()
	})
</script>

<div class={themeState.theme}>
	<ClerkProvider {...data} publishableKey={PUBLIC_CLERK_PUBLISHABLE_KEY}>
		<SignIn />
		<div
			class="bg-stone-50 text-stone-950 dark:text-white dark:bg-zinc-900 min-h-screen scroll-smooth"
		>
			{#if dev}
				<Debug />
			{/if}
			{@render children()}
		</div>
	</ClerkProvider>
</div>
