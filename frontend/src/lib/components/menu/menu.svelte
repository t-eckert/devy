<script lang="ts">
	import HamburgerMenu from "$lib/icons/hamburger-menu.svelte"
	import Card from "$lib/components/card.svelte"

	let isOpen = $state(true)
	let { children } = $props()
</script>

<div>
	<button
		onclick={() => {
			isOpen = !isOpen
		}}
		class={`p-1 rounded-md hover:bg-stone-100 transition-colors text-stone-600 hover:text-stone-950 ${isOpen ? "bg-white" : ""} transition-all`}
	>
		<HamburgerMenu />
	</button>

	{#if isOpen}
		<section class="absolute right-16 top-9">
			<Card>
				{@render children()}
			</Card>
		</section>
	{/if}
</div>
