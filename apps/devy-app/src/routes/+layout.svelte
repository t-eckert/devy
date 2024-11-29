<script lang="ts">
	import "../app.css";
	import Main from "$lib/layouts/main.svelte";
	import Footer from "./footer.svelte";

	let { children } = $props();
</script>

<Main>
	{@render children()}
</Main>
<Footer />
