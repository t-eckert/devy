<script lang="ts">
	import "../app.css"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import { onMount } from "svelte"
	import { setSessionState } from "$lib/state/session.svelte"
	import { setLikesState } from "$lib/state/likes.svelte"
	import Debug from "$lib/debug/debug.svelte"
	import { dev } from "$app/environment"

	let theme = $state("light")
	let { data } = $props()

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

<div class={theme}>
	<!-- t-eckert: I don't know how to render the children for a layout -->
	<!-- svelte-ignore slot_element_deprecated -->
	<div class="bg-stone-50 text-stone-950 dark:text-white dark:bg-stone-900 min-h-screen">
		{#if dev}
			<Debug />
		{/if}
		<slot />
	</div>
</div>
