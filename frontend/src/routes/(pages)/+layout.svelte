<script lang="ts">
	import Analytics from "$lib/utils/analytics.svelte"
	import Main from "$lib/layouts/main.svelte"

	import Header from "./header.svelte"
	import Footer from "./footer.svelte"
</script>

<Analytics />
<div class="min-h-screen">
	<Header />
	<Main>
		<slot />
	</Main>
</div>
<Footer />
