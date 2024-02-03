<script lang="ts">
	import "../app.css"
	import ThemeProvider from "$lib/theme-provider/theme-provider.svelte"
	import Header from "$lib/header/header.svelte"
</script>

<ThemeProvider>
	<div class="min-h-screen bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50">
		<Header />
		<slot />
	</div>
</ThemeProvider>
