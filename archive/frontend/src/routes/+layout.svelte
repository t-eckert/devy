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

	import { setLikes } from "$lib/state/likes.svelte"
	import { setBookmarks } from "$lib/state/bookmarks.svelte"
	import { setTheme } from "$lib/state/theme.svelte"
	import { setUser } from "$lib/state/user.svelte"

	const { data, children }: { data: LayoutData; children: Snippet } = $props()
	const { token } = data

	setUser(token)

	const likesState = setLikes(token)
	const bookmarksState = setBookmarks(token)
	const themeState = setTheme("light")

	onMount(async () => {
		await likesState.loadLikes()
		await bookmarksState.loadBookmarks()
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
