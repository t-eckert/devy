<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import ArrowLeft from "$lib/icons/arrow-left.svelte"
	import ArrowRight from "$lib/icons/arrow-right.svelte"

	export let next: () => void = () => {
		console.log("Next")
	}
	export let prev: () => void | undefined
</script>

<div class="flex flex-row items-center">
	{#if prev}
		<Button role="tertiary" onClick={prev}>
			<div class="group-hover:-translate-x-1 transition">
				<ArrowLeft />
			</div>
			<span class="text-sm font-medium">Prev</span>
		</Button>
	{/if}
	<Button role="tertiary" onClick={next}>
		<span class="text-sm font-medium">Next</span>
		<div class="group-hover:translate-x-1 transition">
			<ArrowRight />
		</div>
	</Button>
</div>
