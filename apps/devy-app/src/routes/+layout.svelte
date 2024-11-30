<script lang="ts">
	import "../app.css";
	import "@fontsource-variable/inter";
	import "@fontsource-variable/playfair";
	import "@fontsource-variable/playfair-display";
	import Main from "$lib/layouts/main.svelte";
	import Bounded from "$lib/layouts/bounded.svelte";
	import OgHeader from "$lib/components/og-header.svelte";
	import ThemeProvider from "$lib/components/theme-provider.svelte";
	import { setTheme } from "$lib/state/theme.svelte";
	import Header from "./header.svelte";
	import Footer from "./footer.svelte";
	import PageProps from "$lib/components/page-props.svelte";

	let { children } = $props();

	setTheme();
</script>

<svelte:head>
	<OgHeader
		title="Devy: git push blog"
		description="Devy is a blogging platform based on Markdown and Git. Connect a GitHub repository, add Markdown files, and every push updates your blog."
	/>
</svelte:head>

<div class="min-h-screen scroll-smooth">
	<ThemeProvider>
		<Main>
			<Header />
			<Bounded>
				<PageProps />
				{@render children()}
			</Bounded>
		</Main>
		<Footer />
	</ThemeProvider>
</div>
