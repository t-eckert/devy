<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import { ArrowLeft, ArrowRight } from "radix-icons-svelte"

	export let next: () => void = () => {
		console.log("Next")
	}
	export let prev: () => void | undefined
</script>

<div class="flex flex-row items-center">
	{#if prev}
		<Button role="tertiary" onClick={prev}>
			<ArrowLeft class="group-hover:-translate-x-1 transition" />
			<span class="text-sm font-medium">Prev</span>
		</Button>
	{/if}
	<Button role="tertiary" onClick={next}>
		<span class="text-sm font-medium">Next</span>
		<ArrowRight class="group-hover:translate-x-1 transition" />
	</Button>
</div>
