<script lang="ts">
	import "../app.css"
	import type { LayoutData } from "./$types"
	import ThemeProvider from "$lib/theme-provider/theme-provider.svelte"
	import Header from "$lib/components/header/header.svelte"
	import Footer from "$lib/components/footer/footer.svelte"
	import { setSession } from "$lib/auth/session-store"
	import "@fontsource/inter"
	import "@fontsource/space-mono"
	import { browser } from "$app/environment"
	import { page } from "$app/stores"
	import * as Fathom from "fathom-client"
	import { onMount } from "svelte"

	onMount(async () => {
		Fathom.load("SRGLORAK", {
			url: "https://cdn.usefathom.com/script.js",
			includedDomains: ["devy.page"]
		})
	})

	$: $page.url.pathname, browser && Fathom.trackPageview()

	export let data: LayoutData

	if (data?.session !== undefined) {
		setSession(data.session)
	}
</script>

<ThemeProvider>
	<div class="bg-zinc-50 text-zinc-800 dark:bg-zinc-950 dark:text-zinc-50">
		<div class="min-h-screen">
			<Header />
			<slot />
		</div>
		<Footer />
	</div>
</ThemeProvider>
