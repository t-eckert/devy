<script>
	import { page } from "$app/stores"
	import { fly } from "svelte/transition"
	import Card from "$lib/components/card/card.svelte"
</script>

{#if $page.error}
	<div class="w-full mx-4" transition:fly>
		<Card class="w-full border-none">
			<div
				class="px-2 py-0.5 bg-red-100 rounded-t-md font-medium text-sm border border-red-200 text-red-700"
			>
				Error
			</div>

			<div class="p-2 pt-6 border-x border-b rounded-b-md border-stone-100 flex flex-col gap-6">
				<div class="flex flex-col">
					<div class="text-8xl font-mono">{$page.status}</div>

					<div class="-mt-1">{$page.error.message}</div>
				</div>
				<div class="flex flex-row gap-2 items-center">
					<a
						class="px-2 py-0.5 bg-stone-800 rounded text-stone-50 font-medium hover:bg-stone-950 transition-all"
						href="/">Go home</a
					>
					<a class="px-2 py-0.5 rounded text-stone-700 font-medium" href="/feedback"
						>Give feedback</a
					>
				</div>
			</div>
		</Card>
	</div>
{/if}
