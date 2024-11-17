<script lang="ts">
	import "../app.css"
	import "@fontsource/ibm-plex-mono"
	import "@fontsource-variable/inter"
	import "@fontsource-variable/playfair"
	import "@fontsource-variable/playfair-display"
	import type { LayoutData } from "./$types"
	import { onMount, type Snippet } from "svelte"
	import Analytics from "$lib/components/analytics.svelte"
	import Main from "$lib/layouts/main.svelte"
	import Bound from "$lib/layouts/bound.svelte"
	import OgHeader from "$lib/components/og-header.svelte"
	import ThemeProvider from "$lib/components/theme-provider.svelte"
	import { dev } from "$app/environment"
	import Debug from "$lib/debug/debug.svelte"
	import { setSession } from "$lib/state/session.svelte"
	import { setTheme } from "$lib/state/theme.svelte"
	import { setLikes } from "$lib/state/likes.svelte"
	import { setBookmarks } from "$lib/state/bookmarks.svelte"

	import Header from "./header.svelte"
	import Footer from "./footer.svelte"

	interface Props {
		children: Snippet
		data: LayoutData
	}

	let { children, data }: Props = $props()
	let { theme, token } = data

	setSession(token)
	setTheme(theme)
	let likes = setLikes(token)
	let bookmarks = setBookmarks(token)

	onMount(async () => {
		await bookmarks.loadBookmarks()
		await likes.loadLikes()
	})
</script>

<svelte:head>
	<OgHeader
		title="Devy: git push blog"
		description="Devy is a blogging platform based on Markdown and Git. Connect a GitHub repository, add Markdown files, and every push updates your blog."
	/>
</svelte:head>

<Analytics />

<ThemeProvider>
	{#if dev}
		<Debug />
	{/if}

	<div class="min-h-screen scroll-smooth">
		<Main>
			<Header />
			<Bound>
				{@render children()}
			</Bound>
		</Main>
		<Footer />
	</div>
</ThemeProvider>
