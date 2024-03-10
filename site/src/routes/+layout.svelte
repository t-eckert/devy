<script lang="ts">
	import "../app.css"
	import ThemeProvider from "$lib/theme-provider/theme-provider.svelte"
	import Header from "$lib/header/header.svelte"
	import Footer from "$lib/footer/footer.svelte"
	import type { LayoutData } from "./$types"
	import Session from "$lib/auth/session.svelte"
	import { setSession } from "$lib/auth/session-store"

	export let data: LayoutData

	if (data?.session) {
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
