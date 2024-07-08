<script lang="ts">
	import Header from "./header.svelte"
	import Footer from "./footer.svelte"
	import Analytics from "$lib/utils/analytics.svelte"
	import Main from "$lib/layouts/main.svelte"
</script>

<Analytics />
<div class="min-h-screen">
	<Main>
		<Header />
		<slot />
	</Main>
</div>
<Footer />
