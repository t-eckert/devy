<script lang="ts">
	import Bound from "$lib/layouts/bound.svelte"
	import Logo from "$lib/components/logo.svelte"
	import Navigation from "./navigation.svelte"
</script>

<div class="mb-4 border-b border-b-slate-200 py-1 dark:border-b-zinc-700">
	<Bound>
		<div class="flex w-full items-center justify-between">
			<a href="/"><Logo height={18} /></a>
			<Navigation />
		</div>
	</Bound>
</div>
