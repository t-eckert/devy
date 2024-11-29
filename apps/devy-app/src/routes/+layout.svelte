<script lang="ts">
	import "../app.css";
	import Main from "$lib/layouts/main.svelte";
	import Header from "./header.svelte";
	import Footer from "./footer.svelte";

	let { children } = $props();
</script>

<Main>
	<Header />
	{@render children()}
</Main>
<Footer />
