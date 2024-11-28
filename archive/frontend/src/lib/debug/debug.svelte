<script lang="ts">
	import Button from "$lib/components/button.svelte"
	import Dialog from "$lib/components/dialog.svelte"

	import Reader from "$lib/icons/reader.svelte"
	import Cross from "$lib/icons/cross-1.svelte"

	import Theme from "./theme.svelte"
	import User from "./user.svelte"

	let isOpen = $state(false)
</script>

{#if isOpen}
	<Dialog>
		<section
			class="w-full h-1/2 max-w-4xl rounded-md border border-stone-200/70 backdrop-blur-3xl shadow bg-white dark:bg-zinc-800/60 dark:border-zinc-800 dark:shadow-none"
		>
			<div class="px-2 py-1 flex flex-row bg-stone-100 rounded-t-md justify-between">
				<h1 class="font-medium text-xs">Debugger</h1>
				<button
					class="text-stone-700 hover:text-stone-950"
					onclick={() => {
						isOpen = false
					}}><Cross /></button
				>
			</div>
			<div class="m-2 grid grid-cols-2 gap-3 items-start">
				<Theme />
				<User />
			</div>
		</section>
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
