<script lang="ts">
	import Button from "$lib/components/button.svelte"
	import Card from "$lib/components/card.svelte"
	import Dialog from "$lib/components/dialog.svelte"

	import Reader from "$lib/icons/reader.svelte"
	import Cross from "$lib/icons/cross-1.svelte"

	import Likes from "./likes.svelte"
	import Session from "./session.svelte"

	let isOpen = $state(false)
</script>

{#if isOpen}
	<Dialog>
		<div class="w-full max-w-6xl">
			<Card>
				<Button
					onclick={() => {
						isOpen = false
					}}><Cross /></Button
				>
				<Likes />
				<Session />
			</Card>
		</div>
	</Dialog>
{:else}
	<div class="z-50 fixed bottom-3 right-3">
		<Button
			onclick={() => {
				isOpen = true
			}}><Reader /></Button
		>
	</div>
{/if}
