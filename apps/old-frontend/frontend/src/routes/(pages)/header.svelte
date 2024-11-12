<script lang="ts">
	import Home from "$lib/components/home.svelte"
	import Main from "$lib/layouts/main.svelte"
	import Navigation from "./navigation.svelte"
</script>

<div class="border-b border-b-stone-200 dark:border-b-zinc-700 mb-4">
	<Main>
		<header class="pt-2 pb-2 flex flex-row gap-3 justify-between items-center">
			<Home />
			<Navigation />
		</header>
	</Main>
</div>
