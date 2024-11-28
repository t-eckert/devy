<script lang="ts">
	import Heart from "$lib/icons/heart.svelte"
	import HeartFilled from "$lib/icons/heart-filled.svelte"

	import { getLikes } from "$lib/state/likes.svelte"
	import { getSession } from "$lib/state/session.svelte"

	const likesState = getLikes()
	const session = getSession()

	const { entryId, likeCount }: { entryId: string; likeCount: number } = $props()

	likesState.setCount(entryId, likeCount)
	let count = $derived(likesState.getCount(entryId))
	let isLiked = $derived(likesState.isLiked(entryId))

	const onclick = async () => {
		await likesState.toggle(entryId)
	}
</script>

{#if session.isAuthenticated}
	<button class="group mr-2 flex flex-row items-center justify-start gap-1.5 text-sm" {onclick}>
		{#if isLiked}
			<div
				class="aspect-square w-4 text-stone-900 transition-all group-hover:-translate-y-1.5 group-hover:rotate-12 group-hover:scale-110 group-hover:text-red-500 dark:text-zinc-100"
			>
				<HeartFilled />
			</div>
		{:else}
			<div
				class="aspect-square w-4 text-stone-500 transition-all group-hover:-translate-y-1.5 group-hover:rotate-12 group-hover:scale-110 group-hover:text-red-500 dark:text-zinc-200"
			>
				<Heart />
			</div>
		{/if}

		<span class="flex w-8 flex-row justify-start font-medium text-stone-700 dark:text-zinc-300"
			>{count}</span
		>
	</button>
{:else}
	<div class="mr-2 flex select-none flex-row items-center justify-start gap-1.5 text-sm">
		<div class="aspect-square w-4 text-stone-500">
			<Heart />
		</div>
		<span class="flex w-8 flex-row justify-start font-medium text-stone-700 dark:text-zinc-200"
			>{count}</span
		>
	</div>
{/if}
