<script lang="ts">
	import "../app.css"
	import type { LayoutData } from "./$types"
	import ThemeProvider from "$lib/theme-provider/theme-provider.svelte"
	import Header from "$lib/components/header/header.svelte"
	import Footer from "$lib/components/footer/footer.svelte"
	import { setSession } from "$lib/state/session-store"
	import "@fontsource-variable/inter"
	import "@fontsource/space-mono"
	import { browser } from "$app/environment"
	import { page } from "$app/stores"
	import * as Fathom from "fathom-client"
	import { onMount } from "svelte"

	// Analytics
	onMount(async () => {
		Fathom.load("SRGLORAK", {
			url: "https://cdn.usefathom.com/script.js",
			includedDomains: ["devy.page"]
		})
	})
	$: $page.url.pathname, browser && Fathom.trackPageview()

	// Session
	export let data: LayoutData
	if (data?.session !== undefined) {
		setSession(data.session)
	}
</script>

<ThemeProvider>
	<div class="bg-stone-50 text-stone-800 dark:bg-stone-950 dark:text-stone-50">
		<div class="min-h-screen">
			<Header />
			<slot />
		</div>
		<Footer />
	</div>
</ThemeProvider>
