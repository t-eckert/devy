<script lang="ts">
	import Analytics from "$lib/utils/analytics.svelte"
	import Main from "$lib/layouts/main.svelte"
	import OGHeader from "$lib/components/og-header.svelte"

	import Header from "./header.svelte"
	import Footer from "./footer.svelte"
</script>

<svelte:head>
	<title>Devy: git push blog</title>
	<OGHeader
		title="Devy: git push blog"
		description="Devy is a blogging platform based on Markdown and Git. Connect a GitHub repository, add Markdown files, and every push updates your blog."
	/>
</svelte:head>

<Analytics />
<div class="min-h-screen">
	<Header />
	<Main>
		<slot />
	</Main>
</div>
<Footer />
