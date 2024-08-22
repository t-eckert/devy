<script lang="ts">
	import "../app.css"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import { onMount } from "svelte"
	import Debug from "$lib/debug/debug.svelte"
	import { dev } from "$app/environment"

	import { setSessionState } from "$lib/state/session.svelte"
	import { setLikesState } from "$lib/state/likes.svelte"
	import { setThemeState } from "$lib/state/theme.svelte"

	const { data } = $props()

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
	<!-- t-eckert: I don't know how to render the children for a layout -->
	<!-- svelte-ignore slot_element_deprecated -->
	<div
		class="bg-stone-50 text-stone-950 dark:text-white dark:bg-zinc-900 min-h-screen scroll-smooth"
	>
		{#if dev}
			<Debug />
		{/if}
		<slot />
	</div>
</div>
