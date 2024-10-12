<script lang="ts">
	import type { LayoutData } from "./$types"
	import { onMount, type Snippet } from "svelte"

	import "../app.css"

	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import "@fontsource/playfair-display"

	import Debug from "$lib/debug/debug.svelte"
	import { dev } from "$app/environment"
	import { Toaster } from "svelte-french-toast"

	// TODO deprecate session state and use user instead
	import { setSessionState } from "$lib/state/session.svelte"

	import { setLikes } from "$lib/state/likes.svelte"
	import { setThemeState } from "$lib/state/theme.svelte"
	import { setUser } from "$lib/state/user.svelte"

	const { data, children }: { data: LayoutData; children: Snippet } = $props()
	const { token } = data

	setUser(token)

	const likesState = setLikes(token)
	const themeState = setThemeState("light")

	const sessionState = setSessionState()
	if (token) {
		sessionState.setToken(token)
	}

	onMount(async () => {
		console.log("onMount")
		await likesState.loadLikes()
	})
</script>

<div class={themeState.theme}>
	<div
		class="bg-stone-50 text-stone-950 dark:text-white dark:bg-zinc-900 min-h-screen scroll-smooth"
	>
		{#if dev}
			<Debug />
		{/if}
		{@render children()}
	</div>
	<Toaster />
</div>
