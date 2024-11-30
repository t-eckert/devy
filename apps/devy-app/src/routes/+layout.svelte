<script lang="ts">
	import "../app.css";
	import "@fontsource-variable/inter";
	import "@fontsource-variable/playfair";
	import "@fontsource-variable/playfair-display";
	import Main from "$lib/layouts/main.svelte";
	import Bounded from "$lib/layouts/bounded.svelte";
	import ThemeProvider from "$lib/components/theme-provider.svelte";
	import { setTheme } from "$lib/state/theme.svelte";
	import Header from "./header.svelte";
	import Footer from "./footer.svelte";

	let { children } = $props();

	setTheme();
</script>

<ThemeProvider>
	<Main>
		<Header />
		<Bounded>
			{@render children()}
		</Bounded>
	</Main>
	<Footer />
</ThemeProvider>
