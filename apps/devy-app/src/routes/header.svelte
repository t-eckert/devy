<script lang="ts">
	import Bounded from "$lib/layouts/bounded.svelte";
	import Logo from "$lib/components/logo.svelte";
</script>

<div class="mb-4 border-b border-b-stone-200 py-2">
	<Bounded>
		<div class="flex w-full items-center justify-between">
			<a href="/"><Logo height={18} /></a>
		</div>
	</Bounded>
</div>
